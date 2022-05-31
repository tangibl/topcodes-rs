use crate::topcode::TopCode;

/// Default maximum width of a TopCode unit in pixels. This is equivalent to 640 pixels.
const DEFAULT_MAX_UNIT: usize = 80;

/// Loads and scans images for TopCodes.  The algorithm does a single sweep of an image (scanning
/// one horizontal line at a time) looking for TopCode bullseye patterns.  If the pattern matches
/// and the black and white regions meet certain ratio constraints, then the pixel is tested as the
/// center of a candidate TopCode.
pub struct Scanner<'a> {
    /// A raw buffer representing the source image in the RGB8 format. 8-bits per channel.
    image_buffer: &'a [u8],
    /// Image width
    width: usize,
    /// Image height
    height: usize,
    /// Holds processed binary pixel data as a single u32 in the ARGB format.
    data: Vec<u32>,
    /// Candidate code count
    candidate_count: usize,
    /// Number of candidates tested
    tested_count: usize,
    /// Maximum width of a TopCode unit in pixels
    max_unit: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(image_buffer: &'a [u8], width: usize, height: usize) -> Self {
        debug_assert!(
            image_buffer.len() == width * height * 3,
            "Scanner received an image buffer (size={}) that did not match the provided width ({}) and height ({})",
            image_buffer.len(),
            width,
            height
        );

        let mut data: Vec<u32> = Vec::with_capacity(width * height);
        // All pixels assumed to be opaque.
        let alpha = 0xff000000; // 0xff << 24
        for i in 0..(width * height) {
            let (r, g, b) = (
                image_buffer[i] as u32,
                image_buffer[i + 1] as u32,
                image_buffer[i + 2] as u32,
            );
            let element = alpha + (r << 16) + (g << 8) + b;
            data.push(element);
        }

        Self {
            image_buffer,
            width,
            height,
            data,
            candidate_count: 0,
            tested_count: 0,
            max_unit: DEFAULT_MAX_UNIT,
        }
    }

    /// Scan the image and return a list of all TopCodes found in it.
    ///
    /// TODO: This process mutates the scanner. Consider creating an intermediate struct for this
    /// process so that the scanner can be held as an immutable reference, even when scanning.
    fn scan(&mut self) {
        todo!()
    }

    /// Sets the maximum allowable diameter (in pixels) for a TopCode identified by the scanner.
    /// Setting this to a reasonable value for your application will reduce false positives
    /// (recognizing codes that aren't actually there) and improve performance (because fewer
    /// candidate codes will be tested). Setting this value to as low as 50 or 60 pixels could be
    /// advisable for some applications. However, setting the maximum diameter too low will prevent
    /// valid codes from being recognized.
    fn set_max_code_diameter(&mut self, diameter: usize) {
        let f = diameter as f64 / 8.0;
        self.max_unit = f.ceil() as usize;
    }

    /// Binary (thresholded black/white) value for pixel (x, y). Value is either 1 (white) or 0
    /// (black).
    ///
    /// TODO: Consider returning an enum/value with a smaller representation here.
    fn get_bw(&self, x: usize, y: usize) -> u32 {
        // TODO: If `threshold` has not been run, this is invalid since the alpha component should
        // contain the thresholded value. We should use type states as mentioned above to avoid
        // this invalid state.
        let pixel = self.data[y * self.width + x];
        return (pixel >> 24) & 0x01;
    }

    /// Average of thresholded pixels in a 3x3 region around (x, y). Returned value is between 0
    /// (black) and 255 (white).
    fn get_sample_3x3(&self, x: usize, y: usize) -> u8 {
        if x < 1 || x >= self.width - 1 || y < 1 || y >= self.height - 1 {
            return 0;
        }

        let mut sum = 0;
        for j in y - 1..=y + 1 {
            for i in x - 1..=x + 1 {
                let pixel = self.data[j * self.width + i];
                sum += 0xff * (pixel >> 24 & 0x01);
            }
        }

        return (sum / 9) as u8;
    }

    /// Average of thresholded pixels in a 3x3 region around (x, y). Returned value is either 0
    /// (black) or 1 (white).
    ///
    /// TODO: Consider returning an enum/value with a smaller representation here.
    fn get_bw_3x3(&self, x: usize, y: usize) -> u32 {
        if x < 1 || x >= self.width - 1 || y < 1 || y >= self.height - 1 {
            return 0;
        }

        let mut sum = 0;
        for j in y - 1..=y + 1 {
            for i in x - 1..=x + 1 {
                let pixel = self.data[j * self.width + i];
                sum += pixel >> 24 & 0x01;
            }
        }

        if sum >= 5 {
            1
        } else {
            0
        }
    }

    /// Perform Wellner adaptive thresholding to produce binary pixel data. Also mark candidate
    /// SpotCode locations.
    ///
    /// "Adaptive Thresholding for the DigitalDesk"
    /// EuroPARC Technical Report EPC-93-110
    fn threshold(&mut self) {
        todo!()
    }

    /// Scan the image line by line looking for TopCodes.
    fn find_codes(&mut self) -> Vec<TopCode> {
        let mut spots = Vec::new();

        // TODO can we make this method take an immutable reference?
        self.tested_count = 0;
        let mut k = self.width * 2;
        for j in 1..self.height - 2 {
            for i in 0..self.width {
                if (self.data[k] & 0x20000000) > 0 {
                    if (self.data[k - 1] & 0x20000000) > 0
                        && (self.data[k + 1] & 0x20000000) > 0
                        && (self.data[k - self.width] & 0x20000000) > 0
                        && (self.data[k + self.width] & 0x20000000) > 0
                    {
                        if !self.overlaps(&spots, i, j) {
                            self.tested_count += 1;
                            let mut spot = TopCode::default();
                            spot.decode(&self, i, j);
                            if spot.is_valid() {
                                spots.push(spot);
                            }
                        }
                    }
                }
                k += 1;
            }
        }

        spots
    }

    fn overlaps(&self, spots: &Vec<TopCode>, x: usize, y: usize) -> bool {
        for top in spots {
            if top.in_bullseye(x as f64, y as f64) {
                return true;
            }
        }

        false
    }

    /// Counts the number of vertical pixels from (x, y) until a color change is perceived.
    fn y_dist(&self, x: usize, y: usize, d: usize) -> Option<usize> {
        let start = self.get_bw_3x3(x, y);

        let start_range = (y + d).max(1);

        for j in (start_range..self.height - 1).step_by(d) {
            let sample = self.get_bw_3x3(x, j);
            if start + sample == 1 {
                return Some(if d > 0 { j - y } else { y - j });
            }
        }

        None
    }

    /// Counts the number of horizontal pixels from (x, y) until a color change is perceived.
    fn x_dist(&self, x: usize, y: usize, d: usize) -> Option<usize> {
        let start = self.get_bw_3x3(x, y);

        let start_range = (x + d).max(1);

        for i in (start_range..self.width - 1).step_by(d) {
            let sample = self.get_bw_3x3(i, y);
            if start + sample == 1 {
                return Some(if d > 0 { i - x } else { x - i });
            }
        }

        None
    }

    /// For debugging purposes, create a black and white image that shows the result of adaptive
    /// thresholding.
    pub(crate) fn write_thresholding_ppm(&self, name: &str) {
        let mut data = String::new();

        // Magic string for identifying file type (plain PPM)
        data.push_str("P3\n");

        // Dimensions
        data.push_str(&format!("{}\t{}\n", self.width, self.height));

        // Maximum color value (between 0 and 65536)
        data.push_str("255\n");

        for value in &self.data {
            let r = (value >> 16) & 0xff;
            let g = (value >> 8) & 0xff;
            let b = value & 0xff;
            data.push_str(&format!("{} {} {}\n", r, g, b));
        }

        std::fs::write(format!("{}.ppm", name), data)
            .expect("Failed to write thresholding image with name {name}");
    }
}
