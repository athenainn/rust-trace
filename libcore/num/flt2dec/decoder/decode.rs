#![feature(core, core_float, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::decode;
    use core::num::flt2dec::FullDecoded::{self, Nan, Infinite, Zero, Finite};
    use core::num::flt2dec::DecodableFloat;
    use core::num::Float;

    // #[derive(Copy, Clone, Debug, PartialEq)]
    // pub struct Decoded {
    //     /// The scaled mantissa.
    //     pub mant: u64,
    //     /// The lower error range.
    //     pub minus: u64,
    //     /// The upper error range.
    //     pub plus: u64,
    //     /// The shared exponent in base 2.
    //     pub exp: i16,
    //     /// True when the error range is inclusive.
    //     ///
    //     /// In IEEE 754, this is true when the original mantissa was even.
    //     pub inclusive: bool,
    // }

    // #[derive(Copy, Clone, Debug, PartialEq)]
    // pub enum FullDecoded {
    /// Not-a-number.
    //     Nan,
    //     /// Infinities, either positive or negative.
    //     Infinite,
    //     /// Zero, either positive or negative.
    //     Zero,
    //     /// Finite numbers with further decoded fields.
    //     Finite(Decoded),
    // }

    // pub fn decode<T: DecodableFloat>(v: T) -> (/*negative?*/ bool, FullDecoded) {
    //     let (mant, exp, sign) = v.integer_decode();
    //     let even = (mant & 1) == 0;
    //     let decoded = match v.classify() {
    //         FpCategory::Nan => FullDecoded::Nan,
    //         FpCategory::Infinite => FullDecoded::Infinite,
    //         FpCategory::Zero => FullDecoded::Zero,
    //         FpCategory::Subnormal => {
    //             // neighbors: (mant - 2, exp) -- (mant, exp) -- (mant + 2, exp)
    //             // Float::integer_decode always preserves the exponent,
    //             // so the mantissa is scaled for subnormals.
    //             FullDecoded::Finite(Decoded { mant: mant, minus: 1, plus: 1,
    //                                           exp: exp, inclusive: even })
    //         }
    //         FpCategory::Normal => {
    //             let minnorm = <T as DecodableFloat>::min_pos_norm_value().integer_decode();
    //             if mant == minnorm.0 {
    //                 // neighbors: (maxmant, exp - 1) -- (minnormmant, exp) -- (minnormmant + 1, exp)
    //                 // where maxmant = minnormmant * 2 - 1
    //                 FullDecoded::Finite(Decoded { mant: mant << 2, minus: 1, plus: 2,
    //                                               exp: exp - 2, inclusive: even })
    //             } else {
    //                 // neighbors: (mant - 1, exp) -- (mant, exp) -- (mant + 1, exp)
    //                 FullDecoded::Finite(Decoded { mant: mant << 1, minus: 1, plus: 1,
    //                                               exp: exp - 1, inclusive: even })
    //             }
    //         }
    //     };
    //     (sign < 0, decoded)
    // }

    type T = f32; // T: DecodableFloat

    #[test]
    fn decode_test1() {
	let v: T = T::nan();
	let (is_negative, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	assert_eq!(is_negative, false);
	assert_eq!(fulldecoded, Nan);
    }

    #[test]
    fn decode_test2() {
	let v: T = T::infinity();
	let (is_negative, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	assert_eq!(is_negative, false);
	assert_eq!(fulldecoded, Infinite);
    }

    #[test]
    fn decode_test3() {
	let v: T = T::neg_infinity();
	let (is_negative, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	assert_eq!(is_negative, true);
	assert_eq!(fulldecoded, Infinite);
    }

    #[test]
    fn decode_test4() {
	let v: T = T::zero();
	let (is_negative, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	assert_eq!(is_negative, false);
	assert_eq!(fulldecoded, Zero);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn decode_test5() {
	let mut v: T = 0.0;

	unsafe {
	    *(&mut v as *const f32 as *mut u32) =
		0b0_00000000_1111111111111111111111; // denormalized floating point
	}

	let (is_negative, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	assert_eq!(is_negative, false);

	match fulldecoded {
	    Nan => assert!(false),
	    Infinite => assert!(false),
	    Zero => assert!(false),
	    Finite(d) => {
		let mant: u64 = d.mant;
		let minus: u64 = d.minus;
		let plus: u64 = d.plus;
		let exp: i16 = d.exp;
		let inclusive: bool = d.inclusive;

		assert_eq!(mant, 0x00000000007ffffe);
		assert_eq!(minus, 1);
		assert_eq!(plus, 1);
		assert_eq!(exp, 0xff6a);
		assert_eq!(inclusive, (mant & 1) == 0);
	    }
	}
    }

    #[test]
    #[allow(overflowing_literals)]
    fn decode_test6() {
	let v: T = <T as DecodableFloat>::min_pos_norm_value();
	let (is_negative, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	assert_eq!(is_negative, false);

	match fulldecoded {
	    Nan => assert!(false),
	    Infinite => assert!(false),
	    Zero => assert!(false),
	    Finite(d) => {
		let mant: u64 = d.mant;
		let minus: u64 = d.minus;
		let plus: u64 = d.plus;
		let exp: i16 = d.exp;
		let inclusive: bool = d.inclusive;

		assert_eq!(mant, 0x0000000002000000);
		assert_eq!(minus, 1);
		assert_eq!(plus, 2);
		assert_eq!(exp, 0xff69);
		assert_eq!(inclusive, (mant & 1) == 0);
	    }
	}
    }

    #[test]
    #[allow(overflowing_literals)]
    fn decode_test7() {
	let v: T = 68.0;
	let (is_negative, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	assert_eq!(is_negative, false);

	match fulldecoded {
	    Nan => assert!(false),
	    Infinite => assert!(false),
	    Zero => assert!(false),
	    Finite(d) => {
		let mant: u64 = d.mant;
		let minus: u64 = d.minus;
		let plus: u64 = d.plus;
		let exp: i16 = d.exp;
		let inclusive: bool = d.inclusive;

		assert_eq!(mant, 0x0000000001100000);
		assert_eq!(minus, 1);
		assert_eq!(plus, 1);
		assert_eq!(exp, 0xffee);
		assert_eq!(inclusive, (mant & 1) == 0);

		assert_eq!(mant as f32 * 2.0_f32.powi(exp as i32), v);
	    }
	}
    }
}
