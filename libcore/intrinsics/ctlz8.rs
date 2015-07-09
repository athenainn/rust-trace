#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctlz8;

    // pub fn ctlz8(x: u8) -> u8;

    macro_rules! ctlz8_test {
	($value:expr, $BITS:expr) => ({
	    let x: u8 = $value;
	    let result: u8 = unsafe { ctlz8(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctlz8_test1() {
	ctlz8_test!(0x00, 8);
	ctlz8_test!(0x01, 7);
	ctlz8_test!(0x03, 6);
	ctlz8_test!(0x07, 5);
	ctlz8_test!(0x0f, 4);
	ctlz8_test!(0x1f, 3);
	ctlz8_test!(0x3f, 2);
	ctlz8_test!(0x7f, 1);
	ctlz8_test!(0xff, 0);
    }
}
