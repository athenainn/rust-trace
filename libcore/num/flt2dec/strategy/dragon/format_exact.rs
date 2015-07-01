#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::MAX_SIG_DIGITS;
    use core::num::flt2dec::strategy::dragon::format_exact;

    use core::num::flt2dec::FullDecoded::{self, Finite};
    use core::num::flt2dec::decode;

    // pub fn format_exact(d: &Decoded, buf: &mut [u8], limit: i16) -> (/*#digits*/ usize, /*exp*/ i16) {
    //     assert!(d.mant > 0);
    //     assert!(d.minus > 0);
    //     assert!(d.plus > 0);
    //     assert!(d.mant.checked_add(d.plus).is_some());
    //     assert!(d.mant.checked_sub(d.minus).is_some());
    //
    //     // estimate `k_0` from original inputs satisfying `10^(k_0-1) < v <= 10^(k_0+1)`.
    //     let mut k = estimate_scaling_factor(d.mant, d.exp);
    //
    //     // `v = mant / scale`.
    //     let mut mant = Big::from_u64(d.mant);
    //     let mut scale = Big::from_small(1);
    //     if d.exp < 0 {
    //         scale.mul_pow2(-d.exp as usize);
    //     } else {
    //         mant.mul_pow2(d.exp as usize);
    //     }
    //
    //     // divide `mant` by `10^k`. now `scale / 10 < mant <= scale * 10`.
    //     if k >= 0 {
    //         mul_pow10(&mut scale, k as usize);
    //     } else {
    //         mul_pow10(&mut mant, -k as usize);
    //     }
    //
    //     // fixup when `mant + plus >= scale`, where `plus / scale = 10^-buf.len() / 2`.
    //     // in order to keep the fixed-size bignum, we actually use `mant + floor(plus) >= scale`.
    //     // we are not actually modifying `scale`, since we can skip the initial multiplication instead.
    //     // again with the shortest algorithm, `d[0]` can be zero but will be eventually rounded up.
    //     if *div_2pow10(&mut scale.clone(), buf.len()).add(&mant) >= scale {
    //         // equivalent to scaling `scale` by 10
    //         k += 1;
    //     } else {
    //         mant.mul_small(10);
    //     }
    //
    //     // if we are working with the last-digit limitation, we need to shorten the buffer
    //     // before the actual rendering in order to avoid double rounding.
    //     // note that we have to enlarge the buffer again when rounding up happens!
    //     let mut len = if k < limit {
    //         // oops, we cannot even produce *one* digit.
    //         // this is possible when, say, we've got something like 9.5 and it's being rounded to 10.
    //         // we return an empty buffer, with an exception of the later rounding-up case
    //         // which occurs when `k == limit` and has to produce exactly one digit.
    //         0
    //     } else if ((k as i32 - limit as i32) as usize) < buf.len() {
    //         (k - limit) as usize
    //     } else {
    //         buf.len()
    //     };
    //
    //     if len > 0 {
    //         // cache `(2, 4, 8) * scale` for digit generation.
    //         // (this can be expensive, so do not calculate them when the buffer is empty.)
    //         let mut scale2 = scale.clone(); scale2.mul_pow2(1);
    //         let mut scale4 = scale.clone(); scale4.mul_pow2(2);
    //         let mut scale8 = scale.clone(); scale8.mul_pow2(3);
    //
    //         for i in 0..len {
    //             if mant.is_zero() { // following digits are all zeroes, we stop here
    //                 // do *not* try to perform rounding! rather, fill remaining digits.
    //                 for c in &mut buf[i..len] { *c = b'0'; }
    //                 return (len, k);
    //             }
    //
    //             let mut d = 0;
    //             if mant >= scale8 { mant.sub(&scale8); d += 8; }
    //             if mant >= scale4 { mant.sub(&scale4); d += 4; }
    //             if mant >= scale2 { mant.sub(&scale2); d += 2; }
    //             if mant >= scale  { mant.sub(&scale);  d += 1; }
    //             debug_assert!(mant < scale);
    //             debug_assert!(d < 10);
    //             buf[i] = b'0' + d;
    //             mant.mul_small(10);
    //         }
    //     }
    //
    //     // rounding up if we stop in the middle of digits
    //     // if the following digits are exactly 5000..., check the prior digit and try to
    //     // round to even (i.e. avoid rounding up when the prior digit is even).
    //     let order = mant.cmp(scale.mul_small(5));
    //     if order == Ordering::Greater || (order == Ordering::Equal &&
    //                                       (len == 0 || buf[len-1] & 1 == 1)) {
    //         // if rounding up changes the length, the exponent should also change.
    //         // but we've been requested a fixed number of digits, so do not alter the buffer...
    //         if let Some(c) = round_up(buf, len) {
    //             // ...unless we've been requested the fixed precision instead.
    //             // we also need to check that, if the original buffer was empty,
    //             // the additional digit can only be added when `k == limit` (edge case).
    //             k += 1;
    //             if k > limit && len < buf.len() {
    //                 buf[len] = c;
    //                 len += 1;
    //             }
    //         }
    //     }
    //
    //     (len, k)
    // }

    type T = f32; // T: DecodableFloat

    #[test]
    fn format_exact_test1() {
	let v: T = 3.141592654;
	let (_, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	match fulldecoded {
	    Finite(d) => {
		let mut buf: [u8; MAX_SIG_DIGITS] = [
		    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		    0, 0, 0, 0, 0, 0, 0
		];
		let limit: i16 = 0;
		let (digits, exp): (usize, i16) = format_exact(&d, &mut buf, limit);

		assert_eq!(digits, 1);
		assert_eq!(exp, 1); // 0 + 1
		assert_eq!(buf, [
		    b'3', 0, 0, 0, 0, 0, 0, 0, 0, 0,
		       0, 0, 0, 0, 0, 0, 0
		]);
	    }
	    _ => assert!(false)
	}
    }

    #[test]
    fn format_exact_test2() {
	let v: T = 3.141592654;
	let (_, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	match fulldecoded {
	    Finite(d) => {
		let mut buf: [u8; MAX_SIG_DIGITS] = [
		    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		    0, 0, 0, 0, 0, 0, 0
		];
		let limit: i16 = -5;
		let (digits, exp): (usize, i16) = format_exact(&d, &mut buf, limit);

		assert_eq!(digits, 6);
		assert_eq!(exp, 1); // -5 + 6
		assert_eq!(buf, [
		    b'3', b'1', b'4', b'1', b'5', b'9', 0, 0, 0, 0,
		       0,    0,    0,    0,    0,    0, 0
		]);
	    }
	    _ => assert!(false)
	}
    }

    #[test]
    fn format_exact_test3() {
	let v: T = 6.022e23;
	let (_, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	match fulldecoded {
	    Finite(d) => {
		let mut buf: [u8; MAX_SIG_DIGITS] = [
		    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		    0, 0, 0, 0, 0, 0, 0
		];
		let limit: i16 = 20;
		let (digits, exp): (usize, i16) = format_exact(&d, &mut buf, limit);

		assert_eq!(digits, 4);
		assert_eq!(exp, 24); // 20 + 4
		assert_eq!(buf, [
		    b'6', b'0', b'2', b'2', 0, 0, 0, 0, 0, 0,
		       0,    0,    0,    0, 0, 0, 0
		]);
	    }
	    _ => assert!(false)
	}
    }


    #[test]
    fn format_exact_test4() {
	let v: T = 6.022e23;
	let (_, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	match fulldecoded {
	    Finite(d) => {
		let mut buf: [u8; MAX_SIG_DIGITS] = [
		    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		    0, 0, 0, 0, 0, 0, 0
		];
		let limit: i16 = 24;
		let (digits, exp): (usize, i16) = format_exact(&d, &mut buf, limit);

		assert_eq!(digits, 1);
		assert_eq!(exp, 25); // 24 + 1
		assert_eq!(buf, [
		    b'1', 0, 0, 0, 0, 0, 0, 0, 0, 0,
		       0, 0, 0, 0, 0, 0, 0
		]);
	    }
	    _ => assert!(false)
	}
    }
}
