#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctpop64;

    // pub fn ctpop64(x: u64) -> u64;

    macro_rules! ctpop64_test {
	($value:expr, $BITS:expr) => ({
	    let x: u64 = $value;
	    let result: u64 = unsafe { ctpop64(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctpop64_test1() {
	ctpop64_test!(0x0000000000000000, 0);
	ctpop64_test!(0x0000000000000001, 1);
	ctpop64_test!(0x0000000000000003, 2);
	ctpop64_test!(0x0000000000000007, 3);
	ctpop64_test!(0x000000000000000f, 4);
	ctpop64_test!(0x000000000000001f, 5);
	ctpop64_test!(0x000000000000003f, 6);
	ctpop64_test!(0x000000000000007f, 7);
	ctpop64_test!(0x00000000000000ff, 8);
	ctpop64_test!(0x00000000000001ff, 9);
	ctpop64_test!(0x00000000000003ff, 10);
	ctpop64_test!(0x00000000000007ff, 11);
	ctpop64_test!(0x0000000000000fff, 12);
	ctpop64_test!(0x0000000000001fff, 13);
	ctpop64_test!(0x0000000000003fff, 14);
	ctpop64_test!(0x0000000000007fff, 15);
	ctpop64_test!(0x000000000000ffff, 16);
	ctpop64_test!(0x000000000001ffff, 17);
	ctpop64_test!(0x000000000003ffff, 18);
	ctpop64_test!(0x000000000007ffff, 19);
	ctpop64_test!(0x00000000000fffff, 20);
	ctpop64_test!(0x00000000001fffff, 21);
	ctpop64_test!(0x00000000003fffff, 22);
	ctpop64_test!(0x00000000007fffff, 23);
	ctpop64_test!(0x0000000000ffffff, 24);
	ctpop64_test!(0x0000000001ffffff, 25);
	ctpop64_test!(0x0000000003ffffff, 26);
	ctpop64_test!(0x0000000007ffffff, 27);
	ctpop64_test!(0x000000000fffffff, 28);
	ctpop64_test!(0x000000001fffffff, 29);
	ctpop64_test!(0x000000003fffffff, 30);
	ctpop64_test!(0x000000007fffffff, 31);
	ctpop64_test!(0x00000000ffffffff, 32);
	ctpop64_test!(0x00000001ffffffff, 33);
	ctpop64_test!(0x00000003ffffffff, 34);
	ctpop64_test!(0x00000007ffffffff, 35);
	ctpop64_test!(0x0000000fffffffff, 36);
	ctpop64_test!(0x0000001fffffffff, 37);
	ctpop64_test!(0x0000003fffffffff, 38);
	ctpop64_test!(0x0000007fffffffff, 39);
	ctpop64_test!(0x000000ffffffffff, 40);
	ctpop64_test!(0x000001ffffffffff, 41);
	ctpop64_test!(0x000003ffffffffff, 42);
	ctpop64_test!(0x000007ffffffffff, 43);
	ctpop64_test!(0x00000fffffffffff, 44);
	ctpop64_test!(0x00001fffffffffff, 45);
	ctpop64_test!(0x00003fffffffffff, 46);
	ctpop64_test!(0x00007fffffffffff, 47);
	ctpop64_test!(0x0000ffffffffffff, 48);
	ctpop64_test!(0x0001ffffffffffff, 49);
	ctpop64_test!(0x0003ffffffffffff, 50);
	ctpop64_test!(0x0007ffffffffffff, 51);
	ctpop64_test!(0x000fffffffffffff, 52);
	ctpop64_test!(0x001fffffffffffff, 53);
	ctpop64_test!(0x003fffffffffffff, 54);
	ctpop64_test!(0x007fffffffffffff, 55);
	ctpop64_test!(0x00ffffffffffffff, 56);
	ctpop64_test!(0x01ffffffffffffff, 57);
	ctpop64_test!(0x03ffffffffffffff, 58);
	ctpop64_test!(0x07ffffffffffffff, 59);
	ctpop64_test!(0x0fffffffffffffff, 60);
	ctpop64_test!(0x1fffffffffffffff, 61);
	ctpop64_test!(0x3fffffffffffffff, 62);
	ctpop64_test!(0x7fffffffffffffff, 63);
	ctpop64_test!(0xffffffffffffffff, 64);
    }
}
