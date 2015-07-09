#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctlz16;

    // pub fn ctlz16(x: u16) -> u16;

    macro_rules! ctlz16_test {
	($value:expr, $BITS:expr) => ({
	    let x: u16 = $value;
	    let result: u16 = unsafe { ctlz16(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctlz16_test1() {
	ctlz16_test!(0x0000, 16);
	ctlz16_test!(0x0001, 15);
	ctlz16_test!(0x0003, 14);
	ctlz16_test!(0x0007, 13);
	ctlz16_test!(0x000f, 12);
	ctlz16_test!(0x001f, 11);
	ctlz16_test!(0x003f, 10);
	ctlz16_test!(0x007f, 9);
	ctlz16_test!(0x00ff, 8);
	ctlz16_test!(0x01ff, 7);
	ctlz16_test!(0x03ff, 6);
	ctlz16_test!(0x07ff, 5);
	ctlz16_test!(0x0fff, 4);
	ctlz16_test!(0x1fff, 3);
	ctlz16_test!(0x3fff, 2);
	ctlz16_test!(0x7fff, 1);
	ctlz16_test!(0xffff, 0);
    }
}
