use crate::topcode::SECTORS;

/// Debug method that prints the 13 least significant bits of an integer.
pub(crate) fn print_bits(bits: isize) -> String {
    let mut lsb = String::new();

    for i in (0..SECTORS).rev() {
        if ((bits >> 1) & 0x01) == 1 {
            lsb.push('1');
        } else {
            lsb.push('0');
        }
        if (44 - i) % 4 == 0 {
            lsb.push(' ');
        }
    }

    format!("{}= {}", lsb, bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_print_the_bits_of_the_13_least_significant_bits() {
        assert_eq!("1 1111 1111 1111 = 31", print_bits(31))
    }
}
