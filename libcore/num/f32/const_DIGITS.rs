#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::DIGITS;

    // pub const DIGITS: u32 = 6;

    #[test]
    fn digits_test1() {
	assert_eq!(DIGITS, 6);
    }
}
