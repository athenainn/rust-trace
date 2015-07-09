#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::cttz16;

    // pub fn cttz16(x: u16) -> u16;

    macro_rules! cttz16_test {
	($value:expr, $BITS:expr) => ({
	    let x: u16 = $value;
	    let result: u16 = unsafe { cttz16(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn cttz16_test1() {
	cttz16_test!(0x0001, 0);
	cttz16_test!(0x0002, 1);
	cttz16_test!(0x0004, 2);
	cttz16_test!(0x0008, 3);
	cttz16_test!(0x0010, 4);
	cttz16_test!(0x0020, 5);
	cttz16_test!(0x0040, 6);
	cttz16_test!(0x0080, 7);
	cttz16_test!(0x0100, 8);
	cttz16_test!(0x0200, 9);
	cttz16_test!(0x0400, 10);
	cttz16_test!(0x0800, 11);
	cttz16_test!(0x1000, 12);
	cttz16_test!(0x2000, 13);
	cttz16_test!(0x4000, 14);
	cttz16_test!(0x8000, 15);
    }
}
