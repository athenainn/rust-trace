#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::RADIX;

    // pub const RADIX: u32 = 2;

    #[test]
    fn radix_test1() {
	assert_eq!(RADIX, 2);
    }
}
