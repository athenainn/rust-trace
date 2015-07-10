#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::cttz8;

    // pub fn cttz8(x: u8) -> u8;

    macro_rules! cttz8_test {
	($value:expr, $BITS:expr) => ({
	    let x: u8 = $value;
	    let result: u8 = unsafe { cttz8(x) };

	    assert_eq!(result, $BITS);
	})
    }

    #[test]
    fn cttz8_test1() {
	cttz8_test!(0x01, 0);
	cttz8_test!(0x02, 1);
	cttz8_test!(0x04, 2);
	cttz8_test!(0x08, 3);
	cttz8_test!(0x10, 4);
	cttz8_test!(0x20, 5);
	cttz8_test!(0x40, 6);
	cttz8_test!(0x80, 7);
    }
}
