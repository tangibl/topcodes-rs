use std::borrow::Cow;

use crate::topcode::SECTORS;

/// Debug method that prints the 13 least significant bits of an integer.
pub(crate) fn print_bits(bits: isize) -> String {
    let mut lsb = String::new();

    for i in (0..SECTORS).rev() {
        if ((bits >> 1) & 0x01) == 1 {
            lsb.push_str("1");
        } else {
            lsb.push_str("0");
        }
        if (44 - i) % 4 == 0 {
            lsb.push_str(" ");
        }
    }

    format!("{} = {}", lsb, bits)
}
