#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::bignum::Digit32;
    use core::num::flt2dec::bignum::Big32x36;

    use core::num::flt2dec::strategy::dragon::mul_pow10;

    // use num::flt2dec::bignum::Digit32 as Digit;
    // use num::flt2dec::bignum::Big32x36 as Big;

    // static POW10: [Digit; 10] = [1, 10, 100, 1000, 10000, 100000,
    //                              1000000, 10000000, 100000000, 1000000000];
    // static TWOPOW10: [Digit; 10] = [2, 20, 200, 2000, 20000, 200000,
    //                                 2000000, 20000000, 200000000, 2000000000];

    // // precalculated arrays of `Digit`s for 10^(2^n)
    // static POW10TO16: [Digit; 2] = [0x6fc10000, 0x2386f2];
    // static POW10TO32: [Digit; 4] = [0, 0x85acef81, 0x2d6d415b, 0x4ee];
    // static POW10TO64: [Digit; 7] = [0, 0, 0xbf6a1f01, 0x6e38ed64, 0xdaa797ed, 0xe93ff9f4, 0x184f03];
    // static POW10TO128: [Digit; 14] =
    //     [0, 0, 0, 0, 0x2e953e01, 0x3df9909, 0xf1538fd, 0x2374e42f, 0xd3cff5ec, 0xc404dc08,
    //      0xbccdb0da, 0xa6337f19, 0xe91f2603, 0x24e];
    // static POW10TO256: [Digit; 27] =
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0x982e7c01, 0xbed3875b, 0xd8d99f72, 0x12152f87, 0x6bde50c6,
    //      0xcf4a6e70, 0xd595d80f, 0x26b2716e, 0xadc666b0, 0x1d153624, 0x3c42d35a, 0x63ff540e,
    //      0xcc5573c0, 0x65f9ef17, 0x55bc28f2, 0x80dcc7f7, 0xf46eeddc, 0x5fdcefce, 0x553f7];

    // pub fn mul_pow10<'a>(x: &'a mut Big, n: usize) -> &'a mut Big {
    //     debug_assert!(n < 512);
    //     if n &   7 != 0 { x.mul_small(POW10[n & 7]); }
    //     if n &   8 != 0 { x.mul_small(POW10[8]); }
    //     if n &  16 != 0 { x.mul_digits(&POW10TO16); }
    //     if n &  32 != 0 { x.mul_digits(&POW10TO32); }
    //     if n &  64 != 0 { x.mul_digits(&POW10TO64); }
    //     if n & 128 != 0 { x.mul_digits(&POW10TO128); }
    //     if n & 256 != 0 { x.mul_digits(&POW10TO256); }
    //     x
    // }

    #[test]
    fn mul_pow10_test1() {
	let v: Digit32 = 1;
	let mut x: Big32x36 = Big32x36::from_small(v);
	let n: usize = 346;
	let mul_pow10: &mut Big32x36 = mul_pow10(&mut x, n);

	let result: String = format!("{:?}", mul_pow10);
	assert_eq!(result, "0x\
	    29d95747_4840e447_dbe38463_c3c38865_\
	    7afb765f_2841d169_38fda647_e6d2e98e_\
	    b50c562d_8166f5a8_6ac48839_a5caf2f1_\
	    258c4359_31dce094_f80e84ce_eaa0be8b_\
	    616dba8b_366e0772_3259dc38_60d63646_\
	    4b4d106b_649a9200_6634e2f7_2c92f806_\
	    584eacde_e4000000_00000000_00000000_\
	    00000000_00000000_00000000_00000000_\
	    00000000_00000000_00000000_00000000".to_string());
    }

    #[test]
    #[should_panic]
    fn mul_pow10_test2() {
	let v: Digit32 = 1;
	let mut x: Big32x36 = Big32x36::from_small(v);
	let n: usize = 347;
	let _: &mut Big32x36 = mul_pow10(&mut x, n); // panicked at 'assertion failed: index < self.len()'
    }
}
