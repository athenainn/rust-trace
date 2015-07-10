#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctpop32;

    // pub fn ctpop32(x: u32) -> u32;

    macro_rules! ctpop32_test {
	($value:expr, $BITS:expr) => ({
	    let x: u32 = $value;
	    let result: u32 = unsafe { ctpop32(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctpop32_test1() {
	ctpop32_test!(0x00000000, 0);
	ctpop32_test!(0x00000001, 1);
	ctpop32_test!(0x00000003, 2);
	ctpop32_test!(0x00000007, 3);
	ctpop32_test!(0x0000000f, 4);
	ctpop32_test!(0x0000001f, 5);
	ctpop32_test!(0x0000003f, 6);
	ctpop32_test!(0x0000007f, 7);
	ctpop32_test!(0x000000ff, 8);
	ctpop32_test!(0x000001ff, 9);
	ctpop32_test!(0x000003ff, 10);
	ctpop32_test!(0x000007ff, 11);
	ctpop32_test!(0x00000fff, 12);
	ctpop32_test!(0x00001fff, 13);
	ctpop32_test!(0x00003fff, 14);
	ctpop32_test!(0x00007fff, 15);
	ctpop32_test!(0x0000ffff, 16);
	ctpop32_test!(0x0001ffff, 17);
	ctpop32_test!(0x0003ffff, 18);
	ctpop32_test!(0x0007ffff, 19);
	ctpop32_test!(0x000fffff, 20);
	ctpop32_test!(0x001fffff, 21);
	ctpop32_test!(0x003fffff, 22);
	ctpop32_test!(0x007fffff, 23);
	ctpop32_test!(0x00ffffff, 24);
	ctpop32_test!(0x01ffffff, 25);
	ctpop32_test!(0x03ffffff, 26);
	ctpop32_test!(0x07ffffff, 27);
	ctpop32_test!(0x0fffffff, 28);
	ctpop32_test!(0x1fffffff, 29);
	ctpop32_test!(0x3fffffff, 30);
	ctpop32_test!(0x7fffffff, 31);
	ctpop32_test!(0xffffffff, 32);
    }
}
