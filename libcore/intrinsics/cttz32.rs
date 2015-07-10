#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::cttz32;

    // pub fn cttz32(x: u32) -> u32;

    macro_rules! cttz32_test {
	($value:expr, $BITS:expr) => ({
	    let x: u32 = $value;
	    let result: u32 = unsafe { cttz32(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn cttz32_test1() {
	cttz32_test!(0x00000001, 0);
	cttz32_test!(0x00000002, 1);
	cttz32_test!(0x00000004, 2);
	cttz32_test!(0x00000008, 3);
	cttz32_test!(0x00000010, 4);
	cttz32_test!(0x00000020, 5);
	cttz32_test!(0x00000040, 6);
	cttz32_test!(0x00000080, 7);
	cttz32_test!(0x00000100, 8);
	cttz32_test!(0x00000200, 9);
	cttz32_test!(0x00000400, 10);
	cttz32_test!(0x00000800, 11);
	cttz32_test!(0x00001000, 12);
	cttz32_test!(0x00002000, 13);
	cttz32_test!(0x00004000, 14);
	cttz32_test!(0x00008000, 15);
	cttz32_test!(0x00010000, 16);
	cttz32_test!(0x00020000, 17);
	cttz32_test!(0x00040000, 18);
	cttz32_test!(0x00080000, 19);
	cttz32_test!(0x00100000, 20);
	cttz32_test!(0x00200000, 21);
	cttz32_test!(0x00400000, 22);
	cttz32_test!(0x00800000, 23);
	cttz32_test!(0x01000000, 24);
	cttz32_test!(0x02000000, 25);
	cttz32_test!(0x04000000, 26);
	cttz32_test!(0x08000000, 27);
	cttz32_test!(0x10000000, 28);
	cttz32_test!(0x20000000, 29);
	cttz32_test!(0x40000000, 30);
	cttz32_test!(0x80000000, 31);
    }
}
