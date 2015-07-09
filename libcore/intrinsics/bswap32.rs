#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::bswap32;

    // pub fn bswap32(x: u32) -> u32;

    macro_rules! bswap32_test {
	($value:expr, $reverse:expr) => ({
	    let x: u32 = $value;
	    let result: u32 = unsafe { bswap32(x) };

	    assert_eq!(result, $reverse);
	})
    }

    #[test]
    fn bswap32_test1() {
	bswap32_test!(0x00000000, 0x00000000);
	bswap32_test!(0x00000001, 0x01000000);
	bswap32_test!(0x00000002, 0x02000000);
	bswap32_test!(0x00000004, 0x04000000);
	bswap32_test!(0x00000008, 0x08000000);
	bswap32_test!(0x00000010, 0x10000000);
	bswap32_test!(0x00000020, 0x20000000);
	bswap32_test!(0x00000040, 0x40000000);
	bswap32_test!(0x00000080, 0x80000000);
	bswap32_test!(0x00000100, 0x00010000);
	bswap32_test!(0x00000200, 0x00020000);
	bswap32_test!(0x00000400, 0x00040000);
	bswap32_test!(0x00000800, 0x00080000);
	bswap32_test!(0x00001000, 0x00100000);
	bswap32_test!(0x00002000, 0x00200000);
	bswap32_test!(0x00004000, 0x00400000);
	bswap32_test!(0x00008000, 0x00800000);
	bswap32_test!(0x00010000, 0x00000100);
	bswap32_test!(0x00020000, 0x00000200);
	bswap32_test!(0x00040000, 0x00000400);
	bswap32_test!(0x00080000, 0x00000800);
	bswap32_test!(0x00100000, 0x00001000);
	bswap32_test!(0x00200000, 0x00002000);
	bswap32_test!(0x00400000, 0x00004000);
	bswap32_test!(0x00800000, 0x00008000);
	bswap32_test!(0x01000000, 0x00000001);
	bswap32_test!(0x02000000, 0x00000002);
	bswap32_test!(0x04000000, 0x00000004);
	bswap32_test!(0x08000000, 0x00000008);
	bswap32_test!(0x10000000, 0x00000010);
	bswap32_test!(0x20000000, 0x00000020);
	bswap32_test!(0x40000000, 0x00000040);
	bswap32_test!(0x80000000, 0x00000080);
    }
}
