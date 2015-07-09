#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctpop16;

    // pub fn ctpop16(x: u16) -> u16;

    macro_rules! ctpop16_test {
	($value:expr, $BITS:expr) => ({
	    let x: u16 = $value;
	    let result: u16 = unsafe { ctpop16(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctpop16_test1() {
	ctpop16_test!(0x0000, 0);
	ctpop16_test!(0x0001, 1);
	ctpop16_test!(0x0003, 2);
	ctpop16_test!(0x0007, 3);
	ctpop16_test!(0x000f, 4);
	ctpop16_test!(0x001f, 5);
	ctpop16_test!(0x003f, 6);
	ctpop16_test!(0x007f, 7);
	ctpop16_test!(0x00ff, 8);
	ctpop16_test!(0x01ff, 9);
	ctpop16_test!(0x03ff, 10);
	ctpop16_test!(0x07ff, 11);
	ctpop16_test!(0x0fff, 12);
	ctpop16_test!(0x1fff, 13);
	ctpop16_test!(0x3fff, 14);
	ctpop16_test!(0x7fff, 15);
	ctpop16_test!(0xffff, 16);
    }
}
