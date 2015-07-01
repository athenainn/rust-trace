#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::MANTISSA_DIGITS;

    // pub const MANTISSA_DIGITS: u32 = 53;

    #[test]
    fn mantissa_digits_test1() {
	assert_eq!(MANTISSA_DIGITS, 53);
    }
}
