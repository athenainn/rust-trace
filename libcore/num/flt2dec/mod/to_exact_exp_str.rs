#![feature(core, flt2dec, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::to_exact_exp_str;
    use core::num::flt2dec::Formatted;
    use core::num::flt2dec::Part::{self, Zero, Num, Copy};
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

    // fn digits_to_exp_str<'a>(buf: &'a [u8], exp: i16, min_ndigits: usize, upper: bool,
    //                          parts: &'a mut [Part<'a>]) -> &'a [Part<'a>] {
    //     assert!(!buf.is_empty());
    //     assert!(buf[0] > b'0');
    //     assert!(parts.len() >= 6);
    //
    //     let mut n = 0;
    //
    //     parts[n] = Part::Copy(&buf[..1]);
    //     n += 1;
    //
    //     if buf.len() > 1 || min_ndigits > 1 {
    //         parts[n] = Part::Copy(b".");
    //         parts[n + 1] = Part::Copy(&buf[1..]);
    //         n += 2;
    //         if min_ndigits > buf.len() {
    //             parts[n] = Part::Zero(min_ndigits - buf.len());
    //             n += 1;
    //         }
    //     }
    //
    //     // 0.1234 x 10^exp = 1.234 x 10^(exp-1)
    //     let exp = exp as i32 - 1; // avoid underflow when exp is i16::MIN
    //     if exp < 0 {
    //         parts[n] = Part::Copy(if upper { b"E-" } else { b"e-" });
    //         parts[n + 1] = Part::Num(-exp as u16);
    //     } else {
    //         parts[n] = Part::Copy(if upper { b"E" } else { b"e" });
    //         parts[n + 1] = Part::Num(exp as u16);
    //     }
    //     &parts[..n + 2]
    // }

    // pub fn to_shortest_exp_str<'a, T, F>(mut format_shortest: F, v: T,
    //                                      sign: Sign, dec_bounds: (i16, i16), upper: bool,
    //                                      buf: &'a mut [u8], parts: &'a mut [Part<'a>]) -> Formatted<'a>
    //         where T: DecodableFloat, F: FnMut(&Decoded, &mut [u8]) -> (usize, i16) {
    //     assert!(parts.len() >= 6);
    //     assert!(buf.len() >= MAX_SIG_DIGITS);
    //     assert!(dec_bounds.0 <= dec_bounds.1);
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
    //             parts[0] = if dec_bounds.0 <= 0 && 0 < dec_bounds.1 {
    //                 Part::Copy(b"0")
    //             } else {
    //                 Part::Copy(if upper { b"0E0" } else { b"0e0" })
    //             };
    //             Formatted { sign: sign, parts: &parts[..1] }
    //         }
    //         FullDecoded::Finite(ref decoded) => {
    //             let (len, exp) = format_shortest(decoded, buf);
    //             let vis_exp = exp as i32 - 1;
    //             let parts = if dec_bounds.0 as i32 <= vis_exp && vis_exp < dec_bounds.1 as i32 {
    //                 digits_to_dec_str(&buf[..len], exp, 0, parts)
    //             } else {
    //                 digits_to_exp_str(&buf[..len], exp, 0, upper, parts)
    //             };
    //             Formatted { sign: sign, parts: parts }
    //         }
    //     }
    // }

    type T = f32;

    #[test]
    fn to_exact_exp_str_test1() {
	let v: T = T::nan();
	let sign: Sign = Minus;
	let ndigits: usize = 1; // dummy
	let upper: bool = false;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("NaN".as_bytes()));
    }

    #[test]
    fn to_exact_exp_str_test2() {
	let v: T = T::infinity();
	let sign: Sign = Minus;
	let ndigits: usize = 1; // dummy
	let upper: bool = false;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("inf".as_bytes()));
    }

    #[test]
    fn to_exact_exp_str_test3() {
	let v: T = T::neg_infinity();
	let sign: Sign = Minus;
	let ndigits: usize = 1; // dummy
	let upper: bool = false;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "-".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("inf".as_bytes()));
    }

    #[test]
    fn to_exact_exp_str_test4() {
	let v: T = T::zero();
	let sign: Sign = Minus;
	let ndigits: usize = 1; // dummy
	let upper: bool = false;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("0e0".as_bytes()));
    }

    #[test]
    fn to_exact_exp_str_test5() {
	let v: T = T::zero();
	let sign: Sign = Minus;
	let ndigits: usize = 1; // dummy
	let upper: bool = true;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 1);
	assert_eq!(formatted.parts[0], Copy("0E0".as_bytes()));
    }

    #[test]
    fn to_exact_exp_str_test6() {
	let v: T = T::zero();
	let sign: Sign = Minus;
	let ndigits: usize = 10;
	let upper: bool = false;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	]; // dummy
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 3);
	assert_eq!(formatted.parts[0], Copy("0.".as_bytes()));
	assert_eq!(formatted.parts[1], Zero(ndigits - 1));
	assert_eq!(formatted.parts[2], Copy("e0".as_bytes()));
    }

    #[test]
    fn to_exact_exp_str_test7() {
	let v: T = 3.141592654;
	let sign: Sign = Minus;
	let ndigits: usize = 15;
	let upper: bool = false;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	];
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 5);
	assert_eq!(formatted.parts[0], Copy("3".as_bytes()));
	assert_eq!(formatted.parts[1], Copy(".".as_bytes()));
	assert_eq!(formatted.parts[2], Copy("14159274101257".as_bytes()));
	assert_eq!(formatted.parts[3], Copy("e".as_bytes()));
	assert_eq!(formatted.parts[4], Num(0));
    }

    #[test]
    fn to_exact_exp_str_test8() {
	let v: T = 6.022e23;
	let sign: Sign = Minus;
	let ndigits: usize = 10;
	let upper: bool = true;
	let mut buf: [u8; MAX_SIG_DIGITS] = [
	    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	    0, 0, 0, 0, 0, 0, 0
	];
	let parts: &mut [Part] = &mut [
	    Zero(0), Zero(0), Zero(0), Zero(0), Zero(0), Zero(0)
	];

	let formatted: Formatted = to_exact_exp_str::<T, _>(
	    strategy::grisu::format_exact,
	    v,
	    sign,
	    ndigits,
	    upper,
	    &mut buf,
	    parts
	);

	assert_eq!(formatted.sign, "".as_bytes());
	assert_eq!(formatted.parts.len(), 5);
	assert_eq!(formatted.parts[0], Copy("6".as_bytes()));
	assert_eq!(formatted.parts[1], Copy(".".as_bytes()));
	assert_eq!(formatted.parts[2], Copy("022000131".as_bytes()));
	assert_eq!(formatted.parts[3], Copy("E".as_bytes()));
	assert_eq!(formatted.parts[4], Num(23));
    }
}
