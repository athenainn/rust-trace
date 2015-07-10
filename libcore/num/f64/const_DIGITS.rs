#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::DIGITS;

    // pub const DIGITS: u32 = 15;

    #[test]
    fn digits_test1() {
	assert_eq!(DIGITS, 15);
    }
}
