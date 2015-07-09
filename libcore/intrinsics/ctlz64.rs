#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ctlz64;

    // pub fn ctlz64(x: u64) -> u64;

    macro_rules! ctlz64_test {
	($value:expr, $BITS:expr) => ({
	    let x: u64 = $value;
	    let result: u64 = unsafe { ctlz64(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn ctlz64_test1() {
	ctlz64_test!(0x0000000000000000, 64);
	ctlz64_test!(0x0000000000000001, 63);
	ctlz64_test!(0x0000000000000003, 62);
	ctlz64_test!(0x0000000000000007, 61);
	ctlz64_test!(0x000000000000000f, 60);
	ctlz64_test!(0x000000000000001f, 59);
	ctlz64_test!(0x000000000000003f, 58);
	ctlz64_test!(0x000000000000007f, 57);
	ctlz64_test!(0x00000000000000ff, 56);
	ctlz64_test!(0x00000000000001ff, 55);
	ctlz64_test!(0x00000000000003ff, 54);
	ctlz64_test!(0x00000000000007ff, 53);
	ctlz64_test!(0x0000000000000fff, 52);
	ctlz64_test!(0x0000000000001fff, 51);
	ctlz64_test!(0x0000000000003fff, 50);
	ctlz64_test!(0x0000000000007fff, 49);
	ctlz64_test!(0x000000000000ffff, 48);
	ctlz64_test!(0x000000000001ffff, 47);
	ctlz64_test!(0x000000000003ffff, 46);
	ctlz64_test!(0x000000000007ffff, 45);
	ctlz64_test!(0x00000000000fffff, 44);
	ctlz64_test!(0x00000000001fffff, 43);
	ctlz64_test!(0x00000000003fffff, 42);
	ctlz64_test!(0x00000000007fffff, 41);
	ctlz64_test!(0x0000000000ffffff, 40);
	ctlz64_test!(0x0000000001ffffff, 39);
	ctlz64_test!(0x0000000003ffffff, 38);
	ctlz64_test!(0x0000000007ffffff, 37);
	ctlz64_test!(0x000000000fffffff, 36);
	ctlz64_test!(0x000000001fffffff, 35);
	ctlz64_test!(0x000000003fffffff, 34);
	ctlz64_test!(0x000000007fffffff, 33);
	ctlz64_test!(0x00000000ffffffff, 32);
	ctlz64_test!(0x00000001ffffffff, 31);
	ctlz64_test!(0x00000003ffffffff, 30);
	ctlz64_test!(0x00000007ffffffff, 29);
	ctlz64_test!(0x0000000fffffffff, 28);
	ctlz64_test!(0x0000001fffffffff, 27);
	ctlz64_test!(0x0000003fffffffff, 26);
	ctlz64_test!(0x0000007fffffffff, 25);
	ctlz64_test!(0x000000ffffffffff, 24);
	ctlz64_test!(0x000001ffffffffff, 23);
	ctlz64_test!(0x000003ffffffffff, 22);
	ctlz64_test!(0x000007ffffffffff, 21);
	ctlz64_test!(0x00000fffffffffff, 20);
	ctlz64_test!(0x00001fffffffffff, 19);
	ctlz64_test!(0x00003fffffffffff, 18);
	ctlz64_test!(0x00007fffffffffff, 17);
	ctlz64_test!(0x0000ffffffffffff, 16);
	ctlz64_test!(0x0001ffffffffffff, 15);
	ctlz64_test!(0x0003ffffffffffff, 14);
	ctlz64_test!(0x0007ffffffffffff, 13);
	ctlz64_test!(0x000fffffffffffff, 12);
	ctlz64_test!(0x001fffffffffffff, 11);
	ctlz64_test!(0x003fffffffffffff, 10);
	ctlz64_test!(0x007fffffffffffff, 9);
	ctlz64_test!(0x00ffffffffffffff, 8);
	ctlz64_test!(0x01ffffffffffffff, 7);
	ctlz64_test!(0x03ffffffffffffff, 6);
	ctlz64_test!(0x07ffffffffffffff, 5);
	ctlz64_test!(0x0fffffffffffffff, 4);
	ctlz64_test!(0x1fffffffffffffff, 3);
	ctlz64_test!(0x3fffffffffffffff, 2);
	ctlz64_test!(0x7fffffffffffffff, 1);
	ctlz64_test!(0xffffffffffffffff, 0);
    }
}
