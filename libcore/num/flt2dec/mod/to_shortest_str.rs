#![feature(core, flt2dec, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::to_shortest_str;
    use core::num::flt2dec::Formatted;
    use core::num::flt2dec::Part::{self, Zero, Copy};
    use core::num::flt2dec::MAX_SIG_DIGITS;
    use core::num::flt2dec::Sign::{self, Minus};

    use core::num::flt2dec::strategy;

    use core::num::Float;

    // #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    // pub enum Part<'a> {
    //     /// Given number of zero digits.
    //     Zero(usize),
    //     /// A literal number up to 5 digits.
    //     Num(u16),
    //     /// A verbatim copy of given bytes.
    //     Copy(&'a [u8]),
    // }

    // #[derive(Clone)]
    // pub struct Formatted<'a> {
    //     /// A byte slice representing a sign, either `""`, `"-"` or `"+"`.
    //     pub sign: &'static [u8],
    //     /// Formatted parts to be rendered after a sign and optional zero padding.
    //     pub parts: &'a [Part<'a>],
    // }

    // #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    // pub enum Sign {
    //     /// Prints `-` only for the negative non-zero values.
    //     Minus,        // -inf -1  0  0  1  inf nan
    //     /// Prints `-` only for any negative values (including the negative zero).
    //     MinusRaw,     // -inf -1 -0  0  1  inf nan
    //     /// Prints `-` for the negative non-zero values, or `+` otherwise.
    //     MinusPlus,    // -inf -1 +0 +0 +1 +inf nan
    //     /// Prints `-` for any negative values (including the negative zero), or `+` otherwise.
    //     MinusPlusRaw, // -inf -1 -0 +0 +1 +inf nan
    // }

    // fn determine_sign(sign: Sign, decoded: &FullDecoded, negative: bool) -> &'static [u8] {
    //     match (*decoded, sign) {
    //         (FullDecoded::Nan, _) => b"",
    //         (FullDecoded::Zero, Sign::Minus) => b"",
    //         (FullDecoded::Zero, Sign::MinusRaw) => if negative { b"-" } else { b"" },
    //         (FullDecoded::Zero, Sign::MinusPlus) => b"+",
    //         (FullDecoded::Zero, Sign::MinusPlusRaw) => if negative { b"-" } else { b"+" },
    //         (_, Sign::Minus) | (_, Sign::MinusRaw) => if negative { b"-" } else { b"" },
    //         (_, Sign::MinusPlus) | (_, Sign::MinusPlusRaw) => if negative { b"-" } else { b"+" },
    //     }
    // }

    // fn digits_to_dec_str<'a>(buf: &'a [u8], exp: i16, frac_digits: usize,
    //                          parts: &'a mut [Part<'a>]) -> &'a [Part<'a>] {
    //     assert!(!buf.is_empty());
    //     assert!(buf[0] > b'0');
    //     assert!(parts.len() >= 4);
    //
    //     // if there is the restriction on the last digit position, `buf` is assumed to be
    //     // left-padded with the virtual zeroes. the number of virtual zeroes, `nzeroes`,
    //     // equals to `max(0, exp + frac_digits - buf.len())`, so that the position of
    //     // the last digit `exp - buf.len() - nzeroes` is no more than `-frac_digits`:
    //     //
    //     //                       |<-virtual->|
    //     //       |<---- buf ---->|  zeroes   |     exp
    //     //    0. 1 2 3 4 5 6 7 8 9 _ _ _ _ _ _ x 10
    //     //    |                  |           |
    //     // 10^exp    10^(exp-buf.len())   10^(exp-buf.len()-nzeroes)
    //     //
    //     // `nzeroes` is individually calculated for each case in order to avoid overflow.
    //
    //     if exp <= 0 {
    //         // the decimal point is before rendered digits: [0.][000...000][1234][____]
    //         let minus_exp = -(exp as i32) as usize;
    //         parts[0] = Part::Copy(b"0.");
    //         parts[1] = Part::Zero(minus_exp);
    //         parts[2] = Part::Copy(buf);
    //         if frac_digits > buf.len() && frac_digits - buf.len() > minus_exp {
    //             parts[3] = Part::Zero((frac_digits - buf.len()) - minus_exp);
    //             &parts[..4]
    //         } else {
    //             &parts[..3]
    //         }
    //     } else {
    //         let exp = exp as usize;
    //         if exp < buf.len() {
    //     //             // the decimal point is inside rendered digits: [12][.][34][____]
    //             parts[0] = Part::Copy(&buf[..exp]);
    //             parts[1] = Part::Copy(b".");
    //             parts[2] = Part::Copy(&buf[exp..]);
    //             if frac_digits > buf.len() - exp {
    //                 parts[3] = Part::Zero(frac_digits - (buf.len() - exp));
    //                 &parts[..4]
    //             } else {
    //                 &parts[..3]
    //             }
    //         } else {
    //             // the decimal point is after rendered digits: [1234][____0000] or [1234][__][.][__].
    //             parts[0] = Part::Copy(buf);
    //             parts[1] = Part::Zero(exp - buf.len());
    //             if frac_digits > 0 {
    //                 parts[2] = Part::Copy(b".");
    //                 parts[3] = Part::Zero(frac_digits);
    //                 &parts[..4]
    //             } else {
    //                 &parts[..2]
    //             }
    //         }
    //     }
    // }

    // pub fn to_shortest_str<'a, T, F>(mut format_shortest: F, v: T,
    //                                  sign: Sign, frac_digits: usize, _upper: bool,
    //                                  buf: &'a mut [u8], parts: &'a mut [Part<'a>]) -> Formatted<'a>
    //         where T: DecodableFloat, F: FnMut(&Decoded, &mut [u8]) -> (usize, i16) {
    //     assert!(parts.len() >= 4);
    //     assert!(buf.len() >= MAX_SIG_DIGITS);
    //
    //     let (negative, full_decoded) = decode(v);
    //     let sign = determine_sign(sign, &full_decoded, negative);
    //     match full_decoded {
    //         FullDecoded::Nan => {
    //             parts[0] = Part::Copy(b"NaN");
    //             Formatted { sign: sign, parts: &parts[..1] }
    //         }
    //         FullDecoded::Infinite => {
    //             parts[0] = Part::Copy(b"inf");
    //             Formatted { sign: sign, parts: &parts[..1] }
    //         }
    //         FullDecoded::Zero => {
    //             if frac_digits > 0 { // [0.][0000]
    //                 parts[0] = Part::Copy(b"0.");
    //                 parts[1] = Part::Zero(frac_digits);
    //                 Formatted { sign: sign, parts: &parts[..2] }
    //             } else {
    //                 parts[0] = Part::Copy(b"0");
    //                 Formatted { sign: sign, parts: &parts[..1] }
    //             }
    //         }
    //         FullDecoded::Finite(ref decoded) => {
    //             let (len, exp) = format_shortest(decoded, buf);
    //             Formatted { sign: sign,
    //                         parts: digits_to_dec_str(&buf[..len], exp, frac_digits, parts) }
    //         }
    //     }
    // }

    type T = f32;

    #[test]
    fn to_shortest_str_test1() {
	let v: T = T::nan();
	let sign: Sign = Minus;
	let frac_digits: usize = 10; // dummy
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("NaN".as_bytes()));
    }

    #[test]
    fn to_shortest_str_test2() {
	let v: T = T::infinity();
	let sign: Sign = Minus;
	let frac_digits: usize = 10; // dummy
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("inf".as_bytes()));
    }

    #[test]
    fn to_shortest_str_test3() {
	let v: T = T::neg_infinity();
	let sign: Sign = Minus;
	let frac_digits: usize = 10; // dummy
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "-".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("inf".as_bytes()));
    }

    #[test]
    fn to_shortest_str_test4() {
	let v: T = T::zero();
	let sign: Sign = Minus;
	let frac_digits: usize = 10;
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 2);
	assert_eq!(formatted.parts[0], Copy("0.".as_bytes()));
	assert_eq!(formatted.parts[1], Zero(frac_digits));
    }

    #[test]
    fn to_shortest_str_test5() {
	let v: T = T::zero();
	let sign: Sign = Minus;
	let frac_digits: usize = 0;
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("0".as_bytes()));
    }

    #[test]
    fn to_shortest_str_test6() {
	let v: T = 68.0;
	let sign: Sign = Minus;
	let frac_digits: usize = 10;
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	];
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 4);
	assert_eq!(formatted.parts[0], Copy("68".as_bytes()));
	assert_eq!(formatted.parts[1], Zero(0));
	assert_eq!(formatted.parts[2], Copy(".".as_bytes()));
	assert_eq!(formatted.parts[3], Zero(frac_digits));
    }

    #[test]
    fn to_shortest_str_test7() {
	let v: T = -68.0;
	let sign: Sign = Minus;
	let frac_digits: usize = 10;
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	];
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "-".as_bytes());
	assert_eq!(formatted.parts.len(), 4);
	assert_eq!(formatted.parts[0], Copy("68".as_bytes()));
	assert_eq!(formatted.parts[1], Zero(0));
	assert_eq!(formatted.parts[2], Copy(".".as_bytes()));
	assert_eq!(formatted.parts[3], Zero(frac_digits));
    }

    #[test]
    fn to_shortest_str_test8() {
	let v: T = 3.141592654;
	let sign: Sign = Minus;
	let frac_digits: usize = 10;
	let _upper: bool = false; // dummy
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	];
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_shortest_str::<T, _>(
	    strategy::grisu::format_shortest,
	    v,
	    sign,
	    frac_digits,
	    _upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 4);
	assert_eq!(formatted.parts[0], Copy("3".as_bytes()));
	assert_eq!(formatted.parts[1], Copy(".".as_bytes()));
	assert_eq!(formatted.parts[2], Copy("1415927".as_bytes()));
	assert_eq!(formatted.parts[3], Zero(3));
    }
}
