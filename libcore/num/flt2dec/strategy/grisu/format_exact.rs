#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::strategy::grisu::format_exact;
    use core::num::flt2dec::decode;
    use core::num::flt2dec::FullDecoded::{self, Finite};
    use core::num::flt2dec::MAX_SIG_DIGITS;

    // pub fn format_exact_opt(d: &Decoded, buf: &mut [u8], limit: i16)
    //                                 -> Option<(/*#digits*/ usize, /*exp*/ i16)> {
    //     assert!(d.mant > 0);
    //     assert!(d.mant < (1 << 61)); // we need at least three bits of additional precision
    //     assert!(!buf.is_empty());
    //
    //     // normalize and scale `v`.
    //     let v = Fp { f: d.mant, e: d.exp }.normalize();
    //     let (minusk, cached) = cached_power(ALPHA - v.e - 64, GAMMA - v.e - 64);
    //     let v = v.mul(&cached);
    //
    //     // divide `v` into integral and fractional parts.
    //     let e = -v.e as usize;
    //     let vint = (v.f >> e) as u32;
    //     let vfrac = v.f & ((1 << e) - 1);
    //
    //     // both old `v` and new `v` (scaled by `10^-k`) has an error of < 1 ulp (Theorem 5.1).
    //     // as we don't know the error is positive or negative, we use two approximations
    //     // spaced equally and have the maximal error of 2 ulps (same to the shortest case).
    //     //
    //     // the goal is to find the exactly rounded series of digits that are common to
    //     // both `v - 1 ulp` and `v + 1 ulp`, so that we are maximally confident.
    //     // if this is not possible, we don't know which one is the correct output for `v`,
    //     // so we give up and fall back.
    //     //
    //     // `err` is defined as `1 ulp * 2^e` here (same to the ulp in `vfrac`),
    //     // and we will scale it whenever `v` gets scaled.
    //     let mut err = 1;
    //
    //     // calculate the largest `10^max_kappa` no more than `v` (thus `v < 10^(max_kappa+1)`).
    //     // this is an upper bound of `kappa` below.
    //     let (max_kappa, max_ten_kappa) = max_pow10_no_more_than(vint);
    //
    //     let mut i = 0;
    //     let exp = max_kappa as i16 - minusk + 1;
    //
    //     // if we are working with the last-digit limitation, we need to shorten the buffer
    //     // before the actual rendering in order to avoid double rounding.
    //     // note that we have to enlarge the buffer again when rounding up happens!
    //     let len = if exp <= limit {
    //         // oops, we cannot even produce *one* digit.
    //         // this is possible when, say, we've got something like 9.5 and it's being rounded to 10.
    //         //
    //         // in principle we can immediately call `possibly_round` with an empty buffer,
    //         // but scaling `max_ten_kappa << e` by 10 can result in overflow.
    //         // thus we are being sloppy here and widen the error range by a factor of 10.
    //         // this will increase the false negative rate, but only very, *very* slightly;
    //         // it can only matter noticably when the mantissa is bigger than 60 bits.
    //         return possibly_round(buf, 0, exp, limit, v.f / 10, (max_ten_kappa as u64) << e, err << e);
    //     } else if ((exp as i32 - limit as i32) as usize) < buf.len() {
    //         (exp - limit) as usize
    //     } else {
    //         buf.len()
    //     };
    //     debug_assert!(len > 0);
    //
    //     // render integral parts.
    //     // the error is entirely fractional, so we don't need to check it in this part.
    //     let mut kappa = max_kappa as i16;
    //     let mut ten_kappa = max_ten_kappa; // 10^kappa
    //     let mut remainder = vint; // digits yet to be rendered
    //     loop { // we always have at least one digit to render
    //         // invariants:
    //         // - `remainder < 10^(kappa+1)`
    //         // - `vint = d[0..n-1] * 10^(kappa+1) + remainder`
    //         //   (it follows that `remainder = vint % 10^(kappa+1)`)
    //
    //         // divide `remainder` by `10^kappa`. both are scaled by `2^-e`.
    //         let q = remainder / ten_kappa;
    //         let r = remainder % ten_kappa;
    //         debug_assert!(q < 10);
    //         buf[i] = b'0' + q as u8;
    //         i += 1;
    //
    //         // is the buffer full? run the rounding pass with the remainder.
    //         if i == len {
    //             let vrem = ((r as u64) << e) + vfrac; // == (v % 10^kappa) * 2^e
    //             return possibly_round(buf, len, exp, limit, vrem, (ten_kappa as u64) << e, err << e);
    //         }
    //
    //         // break the loop when we have rendered all integral digits.
    //         // the exact number of digits is `max_kappa + 1` as `plus1 < 10^(max_kappa+1)`.
    //         if i > max_kappa as usize {
    //             debug_assert_eq!(ten_kappa, 1);
    //             debug_assert_eq!(kappa, 0);
    //             break;
    //         }
    //
    //         // restore invariants
    //         kappa -= 1;
    //         ten_kappa /= 10;
    //         remainder = r;
    //     }
    //
    //     // render fractional parts.
    //     //
    //     // in principle we can continue to the last available digit and check for the accuracy.
    //     // unfortunately we are working with the finite-sized integers, so we need some criterion
    //     // to detect the overflow. V8 uses `remainder > err`, which becomes false when
    //     // the first `i` significant digits of `v - 1 ulp` and `v` differ. however this rejects
    //     // too many otherwise valid input.
    //     //
    //     // since the later phase has a correct overflow detection, we instead use tighter criterion:
    //     // we continue til `err` exceeds `10^kappa / 2`, so that the range between `v - 1 ulp` and
    //     // `v + 1 ulp` definitely contains two or more rounded representations. this is same to
    //     // the first two comparisons from `possibly_round`, for the reference.
    //     let mut remainder = vfrac;
    //     let maxerr = 1 << (e - 1);
    //     while err < maxerr {
    //         // invariants, where `m = max_kappa + 1` (# of digits in the integral part):
    //         // - `remainder < 2^e`
    //         // - `vfrac * 10^(n-m) = d[m..n-1] * 2^e + remainder`
    //         // - `err = 10^(n-m)`
    //
    //         remainder *= 10; // won't overflow, `2^e * 10 < 2^64`
    //         err *= 10; // won't overflow, `err * 10 < 2^e * 5 < 2^64`
    //
    //         // divide `remainder` by `10^kappa`.
    //         // both are scaled by `2^e / 10^kappa`, so the latter is implicit here.
    //         let q = remainder >> e;
    //         let r = remainder & ((1 << e) - 1);
    //         debug_assert!(q < 10);
    //         buf[i] = b'0' + q as u8;
    //         i += 1;
    //
    //         // is the buffer full? run the rounding pass with the remainder.
    //         if i == len {
    //             return possibly_round(buf, len, exp, limit, r, 1 << e, err);
    //         }
    //
    //         // restore invariants
    //         remainder = r;
    //     }
    //
    //     // further calculation is useless (`possibly_round` definitely fails), so we give up.
    //     return None;
    //
    //     // we've generated all requested digits of `v`, which should be also same to corresponding
    //     // digits of `v - 1 ulp`. now we check if there is a unique representation shared by
    //     // both `v - 1 ulp` and `v + 1 ulp`; this can be either same to generated digits, or
    //     // to the rounded-up version of those digits. if the range contains multiple representations
    //     // of the same length, we cannot be sure and should return `None` instead.
    //     //
    //     // all arguments here are scaled by the common (but implicit) value `k`, so that:
    //     // - `remainder = (v % 10^kappa) * k`
    //     // - `ten_kappa = 10^kappa * k`
    //     // - `ulp = 2^-e * k`
    //     fn possibly_round(buf: &mut [u8], mut len: usize, mut exp: i16, limit: i16,
    //                       remainder: u64, ten_kappa: u64, ulp: u64) -> Option<(usize, i16)> {
    //         debug_assert!(remainder < ten_kappa);
    //
    //         //           10^kappa
    //         //    :   :   :<->:   :
    //         //    :   :   :   :   :
    //         //    :|1 ulp|1 ulp|  :
    //         //    :|<--->|<--->|  :
    //         // ----|-----|-----|----
    //         //     |     v     |
    //         // v - 1 ulp   v + 1 ulp
    //         //
    //         // (for the reference, the dotted line indicates the exact value for
    //         // possible representations in given number of digits.)
    //         //
    //         // error is too large that there are at least three possible representations
    //         // between `v - 1 ulp` and `v + 1 ulp`. we cannot determine which one is correct.
    //         if ulp >= ten_kappa { return None; }
    //
    //         //    10^kappa
    //         //   :<------->:
    //         //   :         :
    //         //   : |1 ulp|1 ulp|
    //         //   : |<--->|<--->|
    //         // ----|-----|-----|----
    //         //     |     v     |
    //         // v - 1 ulp   v + 1 ulp
    //         //
    //         // in fact, 1/2 ulp is enough to introduce two possible representations.
    //         // (remember that we need a unique representation for both `v - 1 ulp` and `v + 1 ulp`.)
    //         // this won't overflow, as `ulp < ten_kappa` from the first check.
    //         if ten_kappa - ulp <= ulp { return None; }
    //
    //         //     remainder
    //         //       :<->|                           :
    //         //       :   |                           :
    //         //       :<--------- 10^kappa ---------->:
    //         //     | :   |                           :
    //         //     |1 ulp|1 ulp|                     :
    //         //     |<--->|<--->|                     :
    //         // ----|-----|-----|------------------------
    //         //     |     v     |
    //         // v - 1 ulp   v + 1 ulp
    //         //
    //         // if `v + 1 ulp` is closer to the rounded-down representation (which is already in `buf`),
    //         // then we can safely return. note that `v - 1 ulp` *can* be less than the current
    //         // representation, but as `1 ulp < 10^kappa / 2`, this condition is enough:
    //         // the distance between `v - 1 ulp` and the current representation
    //         // cannot exceed `10^kappa / 2`.
    //         //
    //         // the condition equals to `remainder + ulp < 10^kappa / 2`.
    //         // since this can easily overflow, first check if `remainder < 10^kappa / 2`.
    //         // we've already verified that `ulp < 10^kappa / 2`, so as long as
    //         // `10^kappa` did not overflow after all, the second check is fine.
    //         if ten_kappa - remainder > remainder && ten_kappa - 2 * remainder >= 2 * ulp {
    //             return Some((len, exp));
    //         }
    //
    //         //   :<------- remainder ------>|   :
    //         //   :                          |   :
    //         //   :<--------- 10^kappa --------->:
    //         //   :                    |     |   : |
    //         //   :                    |1 ulp|1 ulp|
    //         //   :                    |<--->|<--->|
    //         // -----------------------|-----|-----|-----
    //         //                        |     v     |
    //         //                    v - 1 ulp   v + 1 ulp
    //         //
    //         // on the other hands, if `v - 1 ulp` is closer to the rounded-up representation,
    //         // we should round up and return. for the same reason we don't need to check `v + 1 ulp`.
    //         //
    //         // the condition equals to `remainder - ulp >= 10^kappa / 2`.
    //         // again we first check if `remainder > ulp` (note that this is not `remainder >= ulp`,
    //         // as `10^kappa` is never zero). also note that `remainder - ulp <= 10^kappa`,
    //         // so the second check does not overflow.
    //         if remainder > ulp && ten_kappa - (remainder - ulp) <= remainder - ulp {
    //             if let Some(c) = round_up(buf, len) {
    //                 // only add an additional digit when we've been requested the fixed precision.
    //                 // we also need to check that, if the original buffer was empty,
    //                 // the additional digit can only be added when `exp == limit` (edge case).
    //                 exp += 1;
    //                 if exp > limit && len < buf.len() {
    //                     buf[len] = c;
    //                     len += 1;
    //                 }
    //             }
    //             return Some((len, exp));
    //         }
    //
    //         // otherwise we are doomed (i.e. some values between `v - 1 ulp` and `v + 1 ulp` are
    //         // rounding down and others are rounding up) and give up.
    //         None
    //     }
    // }

    // pub fn format_exact(d: &Decoded, buf: &mut [u8], limit: i16) -> (/*#digits*/ usize, /*exp*/ i16) {
    //     use num::flt2dec::strategy::dragon::format_exact as fallback;
    //     match format_exact_opt(d, buf, limit) {
    //         Some(ret) => ret,
    //         None => fallback(d, buf, limit),
    //     }
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
