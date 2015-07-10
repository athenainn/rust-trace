#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::bswap16;

    // pub fn bswap16(x: u16) -> u16;

    macro_rules! bswap16_test {
	($value:expr, $reverse:expr) => ({
	    let x: u16 = $value;
	    let result: u16 = unsafe { bswap16(x) };

	    assert_eq!(result, $reverse);
	})
    }

    #[test]
    fn bswap16_test1() {
	bswap16_test!(0x0000, 0x0000);
	bswap16_test!(0x0001, 0x0100);
	bswap16_test!(0x0002, 0x0200);
	bswap16_test!(0x0004, 0x0400);
	bswap16_test!(0x0008, 0x0800);
	bswap16_test!(0x0010, 0x1000);
	bswap16_test!(0x0020, 0x2000);
	bswap16_test!(0x0040, 0x4000);
	bswap16_test!(0x0080, 0x8000);
	bswap16_test!(0x0100, 0x0001);
	bswap16_test!(0x0200, 0x0002);
	bswap16_test!(0x0400, 0x0004);
	bswap16_test!(0x0800, 0x0008);
	bswap16_test!(0x1000, 0x0010);
	bswap16_test!(0x2000, 0x0020);
	bswap16_test!(0x4000, 0x0040);
	bswap16_test!(0x8000, 0x0080);
    }
}
