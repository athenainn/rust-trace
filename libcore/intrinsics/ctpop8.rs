#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctpop8;

    // pub fn ctpop8(x: u8) -> u8;

    macro_rules! ctpop8_test {
	($value:expr, $BITS:expr) => ({
	    let x: u8 = $value;
	    let result: u8 = unsafe { ctpop8(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctpop8_test1() {
	ctpop8_test!(0x00, 0);
	ctpop8_test!(0x01, 1);
	ctpop8_test!(0x03, 2);
	ctpop8_test!(0x07, 3);
	ctpop8_test!(0x0f, 4);
	ctpop8_test!(0x1f, 5);
	ctpop8_test!(0x3f, 6);
	ctpop8_test!(0x7f, 7);
	ctpop8_test!(0xff, 8);
    }
}
