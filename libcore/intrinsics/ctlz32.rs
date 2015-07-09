#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctlz32;

    // pub fn ctlz32(x: u32) -> u32;

    macro_rules! ctlz32_test {
	($value:expr, $BITS:expr) => ({
	    let x: u32 = $value;
	    let result: u32 = unsafe { ctlz32(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctlz32_test1() {
	ctlz32_test!(0x00000000, 32);
	ctlz32_test!(0x00000001, 31);
	ctlz32_test!(0x00000003, 30);
	ctlz32_test!(0x00000007, 29);
	ctlz32_test!(0x0000000f, 28);
	ctlz32_test!(0x0000001f, 27);
	ctlz32_test!(0x0000003f, 26);
	ctlz32_test!(0x0000007f, 25);
	ctlz32_test!(0x000000ff, 24);
	ctlz32_test!(0x000001ff, 23);
	ctlz32_test!(0x000003ff, 22);
	ctlz32_test!(0x000007ff, 21);
	ctlz32_test!(0x00000fff, 20);
	ctlz32_test!(0x00001fff, 19);
	ctlz32_test!(0x00003fff, 18);
	ctlz32_test!(0x00007fff, 17);
	ctlz32_test!(0x0000ffff, 16);
	ctlz32_test!(0x0001ffff, 15);
	ctlz32_test!(0x0003ffff, 14);
	ctlz32_test!(0x0007ffff, 13);
	ctlz32_test!(0x000fffff, 12);
	ctlz32_test!(0x001fffff, 11);
	ctlz32_test!(0x003fffff, 10);
	ctlz32_test!(0x007fffff, 9);
	ctlz32_test!(0x00ffffff, 8);
	ctlz32_test!(0x01ffffff, 7);
	ctlz32_test!(0x03ffffff, 6);
	ctlz32_test!(0x07ffffff, 5);
	ctlz32_test!(0x0fffffff, 4);
	ctlz32_test!(0x1fffffff, 3);
	ctlz32_test!(0x3fffffff, 2);
	ctlz32_test!(0x7fffffff, 1);
	ctlz32_test!(0xffffffff, 0);
    }
}
