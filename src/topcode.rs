use std::{f64::consts::PI, fmt::Display};

use crate::scanner::Scanner;

/// Number of sectors in the data ring
pub const SECTORS: usize = 13;

/// Width of the code in units (ring widths)
const WIDTH: usize = 8;

/// The default diameter for a TopCode
const DEFAULT_DIAMETER: f64 = 72.0;

/// Span of a data sector in radians
const ARC: f64 = 2.0 * PI / (SECTORS as f64);

/// An unsigned integer representing a symbol code of a given TopCode. Since TopCodes never exceed
/// 13 bits in size, a u16 is sufficient.
///
/// This type alias exists simply ensure that if the data type needs to change, this is the only
/// line of code that should have to change.
pub type Code = u16;

/// TopCodes (Tangible Object Placement Codes) are black-and-white circular fiducials designed to
/// be recognized quickly by low-resolution digital cameras with poor optics. The TopCode symmbol
/// format is based on the open SpotCode format:
///
/// https://www.cl.cam.ac.uk/research/srg/netos/projects/archive/uid/spotcode.html
///
/// Each TopCode encodes a 13-bit number in a single data ring on the outer edge of the symbol.
/// Zero is represented by a black sector and one is represented by a white sector.
#[derive(Clone, Copy, Debug)]
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
    pub(crate) core: [isize; WIDTH],
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

    /// Sets the x- and y- coordinates for the center point of the symbol.
    pub fn set_location(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Returns true if the code was successfully decoded.
    pub fn is_valid(&self) -> bool {
        self.code.is_some()
    }

    /// Decodes a symbol given any point (cx, by) inside the center circle (bullseye) of the code.
    pub fn decode(&mut self, scanner: &Scanner, cx: usize, cy: usize) -> Option<Code> {
        todo!()
    }

    /// Attempts to decode the binary pixels of an image into a code value.
    ///
    /// The `unit` is the width of a single ring and `arc_adjustment` corrects the rotation.
    fn read_code(scanner: &Scanner, unit: f64, arc_adjustment: f64) -> Option<Code> {
        todo!()
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
    fn read_unit(scanner: &Scanner) -> f64 {
        todo!()
    }

    #[cfg(feature = "visualize")]
    pub fn annotate(&self) {
        unimplemented!()
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
        assert!(!topcode.in_bullseye(topcode.unit, 0.0));
        assert!(!topcode.in_bullseye(0.0, topcode.unit));
    }
}
