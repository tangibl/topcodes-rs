use std::f64::consts::PI;

use crate::scanner::Scanner;

/// Number of sectors in the data ring
pub const SECTORS: usize = 13;

/// Width of the code in units (ring widths)
const WIDTH: usize = 8;

/// The default diameter for a TopCode
const DEFAULT_DIAMETER: f64 = 72.0;

/// Span of a data sector in radians
const ARC: f64 = 2.0 * PI / (SECTORS as f64);

const MAX_PIXELS: usize = 100;

/// An unsigned integer representing a symbol code of a given TopCode. Since TopCodes never exceed
/// Valid TopCodes are 13 bits in size, but invalid ones may be more, so this is represented as a
/// u32.
///
/// This type alias exists simply ensure that if the data type needs to change, this is the only
/// line of code that should have to change.
pub type Code = u32;

/// TopCodes (Tangible Object Placement Codes) are black-and-white circular fiducials designed to
/// be recognized quickly by low-resolution digital cameras with poor optics. The TopCode symmbol
/// format is based on the open SpotCode format:
///
/// https://www.cl.cam.ac.uk/research/srg/netos/projects/archive/uid/spotcode.html
///
/// Each TopCode encodes a 13-bit number in a single data ring on the outer edge of the symbol.
/// Zero is represented by a black sector and one is represented by a white sector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TopCode {
    /// The symbol's code, if valid
    pub code: Option<Code>,
    /// Width of a single ring
    pub unit: f64,
    /// Angular orientation of the symbol (in radians)
    pub orientation: f64,
    /// Horizontal center of a symbol
    pub x: f64,
    /// Vertical center of a symbol
    pub y: f64,
    /// Buffer used to decode sectors
    pub(crate) core: [usize; WIDTH],
}

impl Default for TopCode {
    fn default() -> Self {
        Self {
            code: None,
            unit: DEFAULT_DIAMETER / WIDTH as f64,
            orientation: 0.0,
            x: 0.0,
            y: 0.0,
            core: [0; WIDTH],
        }
    }
}

impl TopCode {
    /// Create a default TopCode with the given identifier.
    pub fn new(code: Code) -> Self {
        let mut topcode = Self::default();
        topcode.code = Some(code);
        topcode
    }

    pub fn radius(&self) -> f64 {
        self.unit * WIDTH as f64 / 2.0
    }

    /// Sets the x- and y- coordinates for the center point of the symbol.
    pub fn set_location(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Returns true if the code was successfully decoded and is not too close to the edges of the
    /// image.
    pub fn is_valid(&self) -> bool {
        self.code.is_some()
    }

    /// Decodes a symbol given any point (cx, by) inside the center circle (bullseye) of the code.
    pub fn decode(&mut self, scanner: &Scanner, cx: usize, cy: usize) -> Option<Code> {
        let up = scanner.dist(cx, cy, 0, -1);
        let down = scanner.dist(cx, cy, 0, 1);
        let left = scanner.dist(cx, cy, -1, 0);
        let right = scanner.dist(cx, cy, 1, 0);

        self.x = cx as f64;
        self.y = cy as f64;
        self.x += (right - left) as f64 / 6.0;
        self.y += (down - up) as f64 / 6.0;
        self.code = None;
        self.unit = self.read_unit(scanner); // Try to make this an option. Consider a valid vs. invalid TopCode enum.

        if self.unit < 0.0 {
            return None;
        }

        let mut max_c = 0;
        let mut max_a = 0.0;
        let mut max_u = 0.0;

        // Try different unit and arc adjustments. Save the one that produces a maximum confidence
        // reading....
        for u in -2..=2 {
            for a in 0..10 {
                let arc_adjustment = a as f64 * ARC * 0.1;
                let unit = self.unit + (self.unit * 0.05 * u as f64);
                let c = self.read_code(scanner, unit, arc_adjustment);
                if c > max_c {
                    max_c = c;
                    max_a = arc_adjustment;
                    max_u = unit;
                }
            }
        }

        // One last call to [read_code] to reset orientation and code.
        if max_c > 0 {
            self.unit = max_u;
            self.read_code(scanner, self.unit, max_a);
            self.code = self.code.map(|code| self.rotate_lowest(code, max_a));
        }

        self.code
    }

    /// Attempts to decode the binary pixels of an image into a code value.
    ///
    /// The `unit` is the width of a single ring and `arc_adjustment` corrects the rotation.
    fn read_code(&mut self, scanner: &Scanner, unit: f64, arc_adjustment: f64) -> usize {
        let mut c = 0;
        let mut bit = 0;
        let mut bits = 0;

        for sector in (0..SECTORS).rev() {
            let sector_f = sector as f64;
            let dx = (ARC * sector_f + arc_adjustment).cos();
            let dy = (ARC * sector_f + arc_adjustment).sin();

            // Take 8 samples across the diameter of the symbol
            for i in 0..WIDTH {
                let i_f = i as f64;
                let dist = (i_f - 3.5) * unit;

                let sx = (self.x + dx * dist).round() as usize;
                let sy = (self.y + dy * dist).round() as usize;
                self.core[i] = scanner.get_sample_3x3(sx, sy);
            }

            // White rings
            if self.core[1] <= 128
                || self.core[3] <= 128
                || self.core[4] <= 128
                || self.core[6] <= 128
            {
                return 0;
            }

            // Black ring
            if self.core[2] > 128 || self.core[5] > 128 {
                return 0;
            }

            // Compute confidence interval in core sample
            c += self.core[1] // White rings
                + self.core[3]
                + self.core[4]
                + self.core[6]
                + (0xff - self.core[2]) // Black ring
                + (0xff - self.core[5]);

            // Data rings
            c += (self.core[7] as isize * 2 - 0xff).abs() as usize;

            // Opposite data ring
            c += (0xff - (self.core[0] as isize * 2 - 0xff)) as usize;

            bit = if self.core[7] > 128 { 1 } else { 0 };
            bits <<= 1;
            bits += bit;
        }

        return if Self::checksum(bits) {
            self.code = Some(bits);
            c
        } else {
            self.code = None;
            0
        };
    }

    /// Tries each of the possible rotations and returns the lowest.
    fn rotate_lowest(&mut self, mut bits: Code, mut arc_adjustment: f64) -> Code {
        let mut min = bits;
        let mask = 0x1fff;

        arc_adjustment -= ARC * 0.65;

        self.orientation = 0.0;

        for i in 1..=SECTORS {
            bits = ((bits << 1) & mask) | (bits >> (SECTORS - 1));
            if bits < min {
                min = bits;
                self.orientation = i as f64 * -ARC;
            }
        }

        self.orientation += arc_adjustment;
        return min;
    }

    /// Only codes with a checksum of 5 are valid.
    fn checksum(mut bits: Code) -> bool {
        let mut sum = 0;
        for _i in 0..SECTORS {
            sum += bits & 0x01;
            bits = bits >> 1;
        }

        return sum == 5;
    }

    /// Returns true if the given point is inside the bullseye
    pub(crate) fn in_bullseye(&self, px: f64, py: f64) -> bool {
        return ((self.x - px) * (self.x - px) + (self.y - py) * (self.y - py))
            <= (self.unit * self.unit);
    }

    /// Determines the symbol's unit length by counting the number of pixels between the outer
    /// edges of the first black ring. North, south, east, and west readings are taken and the
    /// average is returned.
    fn read_unit(&self, scanner: &Scanner) -> f64 {
        let sx = self.x.round() as usize;
        let sy = self.y.round() as usize;

        let image_width = scanner.image_width();
        let image_height = scanner.image_height();

        let mut white_left = true;
        let mut white_right = true;
        let mut white_up = true;
        let mut white_down = true;

        let mut dist_left = 0;
        let mut dist_right = 0;
        let mut dist_up = 0;
        let mut dist_down = 0;

        for i in 1..=MAX_PIXELS {
            if sx < 1 + i || sx + i >= image_width - 1 || sy < 1 + i || sy + i >= image_height - 1 {
                return -1.0;
            }

            // Left sample
            let sample = scanner.get_bw_3x3(sx - i, sy);
            if dist_left <= 0 {
                if white_left && sample == 0 {
                    white_left = false
                } else if !white_left && sample == 1 {
                    dist_left = i as isize;
                }
            }

            // Right sample
            let sample = scanner.get_bw_3x3(sx + i, sy);
            if dist_right <= 0 {
                if white_right && sample == 0 {
                    white_right = false
                } else if !white_right && sample == 1 {
                    dist_right = i as isize;
                }
            }

            // Up sample
            let sample = scanner.get_bw_3x3(sx, sy - i);
            if dist_up <= 0 {
                if white_up && sample == 0 {
                    white_up = false
                } else if !white_up && sample == 1 {
                    dist_up = i as isize;
                }
            }

            // Down sample
            let sample = scanner.get_bw_3x3(sx, sy + i);
            if dist_down <= 0 {
                if white_down && sample == 0 {
                    white_down = false
                } else if !white_down && sample == 1 {
                    dist_down = i as isize;
                }
            }

            if dist_right > 0 && dist_left > 0 && dist_up > 0 && dist_down > 0 {
                let u = (dist_right + dist_left + dist_up + dist_down) as f64 / 8.0;
                return if (dist_right + dist_left - dist_up - dist_down).abs() as f64 > u {
                    -1.0
                } else {
                    u
                };
            }
        }

        -1.0
    }

    /// A method used to draw the current TopCode. This should only be conditionally compiled for
    /// experimentation and testing. Otherwise, consumers of this library are responsible for
    /// implementing methods to draw the TopCodes.
    #[cfg(feature = "visualize")]
    pub fn draw(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_is_valid() {
        assert!(TopCode::checksum(0b111011));
    }

    #[test]
    fn checksum_is_invalid() {
        assert!(!TopCode::checksum(0b10101));
    }

    #[test]
    fn point_is_in_bullseye() {
        let topcode = TopCode::default();
        assert!(topcode.in_bullseye(0.0, 0.0));
        assert!(topcode.in_bullseye(topcode.unit, 0.0));
        assert!(topcode.in_bullseye(0.0, topcode.unit));
    }

    #[test]
    fn point_is_not_in_bullseye() {
        let topcode = TopCode::default();
        assert!(!topcode.in_bullseye(topcode.unit, topcode.unit));
    }
}
