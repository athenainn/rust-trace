#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::strategy::grisu::format_shortest;
    use core::num::flt2dec::decode;
    use core::num::flt2dec::FullDecoded::Finite;
    use core::num::flt2dec::MAX_SIG_DIGITS;

    // pub fn format_shortest_opt(d: &Decoded,
    //                            buf: &mut [u8]) -> Option<(/*#digits*/ usize, /*exp*/ i16)> {
    //     assert!(d.mant > 0);
    //     assert!(d.minus > 0);
    //     assert!(d.plus > 0);
    //     assert!(d.mant.checked_add(d.plus).is_some());
    //     assert!(d.mant.checked_sub(d.minus).is_some());
    //     assert!(buf.len() >= MAX_SIG_DIGITS);
    //     assert!(d.mant + d.plus < (1 << 61)); // we need at least three bits of additional precision
    //
    //     // start with the normalized values with the shared exponent
    //     let plus = Fp { f: d.mant + d.plus, e: d.exp }.normalize();
    //     let minus = Fp { f: d.mant - d.minus, e: d.exp }.normalize_to(plus.e);
    //     let v = Fp { f: d.mant, e: d.exp }.normalize_to(plus.e);
    //
    //     // find any `cached = 10^minusk` such that `ALPHA <= minusk + plus.e + 64 <= GAMMA`.
    //     // since `plus` is normalized, this means `2^(62 + ALPHA) <= plus * cached < 2^(64 + GAMMA)`;
    //     // given our choices of `ALPHA` and `GAMMA`, this puts `plus * cached` into `[4, 2^32)`.
    //     //
    //     // it is obviously desirable to maximize `GAMMA - ALPHA`,
    //     // so that we don't need many cached powers of 10, but there are some considerations:
    //     //
    //     // 1. we want to keep `floor(plus * cached)` within `u32` since it needs a costly division.
    //     //    (this is not really avoidable, remainder is required for accuracy estimation.)
    //     // 2. the remainder of `floor(plus * cached)` repeatedly gets multiplied by 10,
    //     //    and it should not overflow.
    //     //
    //     // the first gives `64 + GAMMA <= 32`, while the second gives `10 * 2^-ALPHA <= 2^64`;
    //     // -60 and -32 is the maximal range with this constraint, and V8 also uses them.
    //     let (minusk, cached) = cached_power(ALPHA - plus.e - 64, GAMMA - plus.e - 64);
    //
    //     // scale fps. this gives the maximal error of 1 ulp (proved from Theorem 5.1).
    //     let plus = plus.mul(&cached);
    //     let minus = minus.mul(&cached);
    //     let v = v.mul(&cached);
    //     debug_assert_eq!(plus.e, minus.e);
    //     debug_assert_eq!(plus.e, v.e);
    //
    //     //         +- actual range of minus
    //     //   | <---|---------------------- unsafe region --------------------------> |
    //     //   |     |                                                                 |
    //     //   |  |<--->|  | <--------------- safe region ---------------> |           |
    //     //   |  |     |  |                                               |           |
    //     //   |1 ulp|1 ulp|                 |1 ulp|1 ulp|                 |1 ulp|1 ulp|
    //     //   |<--->|<--->|                 |<--->|<--->|                 |<--->|<--->|
    //     //   |-----|-----|-------...-------|-----|-----|-------...-------|-----|-----|
    //     //   |   minus   |                 |     v     |                 |   plus    |
    //     // minus1     minus0           v - 1 ulp   v + 1 ulp           plus0       plus1
    //     //
    //     // above `minus`, `v` and `plus` are *quantized* approximations (error < 1 ulp).
    //     // as we don't know the error is positive or negative, we use two approximations spaced equally
    //     // and have the maximal error of 2 ulps.
    //     //
    //     // the "unsafe region" is a liberal interval which we initially generate.
    //     // the "safe region" is a conservative interval which we only accept.
    //     // we start with the correct repr within the unsafe region, and try to find the closest repr
    //     // to `v` which is also within the safe region. if we can't, we give up.
    //     let plus1 = plus.f + 1;
    // //  let plus0 = plus.f - 1; // only for explanation
    // //  let minus0 = minus.f + 1; // only for explanation
    //     let minus1 = minus.f - 1;
    //     let e = -plus.e as usize; // shared exponent
    //
    //     // divide `plus1` into integral and fractional parts.
    //     // integral parts are guaranteed to fit in u32, since cached power guarantees `plus < 2^32`
    //     // and normalized `plus.f` is always less than `2^64 - 2^4` due to the precision requirement.
    //     let plus1int = (plus1 >> e) as u32;
    //     let plus1frac = plus1 & ((1 << e) - 1);
    //
    //     // calculate the largest `10^max_kappa` no more than `plus1` (thus `plus1 < 10^(max_kappa+1)`).
    //     // this is an upper bound of `kappa` below.
    //     let (max_kappa, max_ten_kappa) = max_pow10_no_more_than(plus1int);
    //
    //     let mut i = 0;
    //     let exp = max_kappa as i16 - minusk + 1;
    //
    //     // Theorem 6.2: if `k` is the greatest integer s.t. `0 <= y mod 10^k <= y - x`,
    //     //              then `V = floor(y / 10^k) * 10^k` is in `[x, y]` and one of the shortest
    //     //              representations (with the minimal number of significant digits) in that range.
    //     //
    //     // find the digit length `kappa` between `(minus1, plus1)` as per Theorem 6.2.
    //     // Theorem 6.2 can be adopted to exclude `x` by requiring `y mod 10^k < y - x` instead.
    //     // (e.g. `x` = 32000, `y` = 32777; `kappa` = 2 since `y mod 10^3 = 777 < y - x = 777`.)
    //     // the algorithm relies on the later verification phase to exclude `y`.
    //     let delta1 = plus1 - minus1;
    // //  let delta1int = (delta1 >> e) as usize; // only for explanation
    //     let delta1frac = delta1 & ((1 << e) - 1);
    //
    //     // render integral parts, while checking for the accuracy at each step.
    //     let mut kappa = max_kappa as i16;
    //     let mut ten_kappa = max_ten_kappa; // 10^kappa
    //     let mut remainder = plus1int; // digits yet to be rendered
    //     loop { // we always have at least one digit to render, as `plus1 >= 10^kappa`
    //         // invariants:
    //         // - `delta1int <= remainder < 10^(kappa+1)`
    //         // - `plus1int = d[0..n-1] * 10^(kappa+1) + remainder`
    //         //   (it follows that `remainder = plus1int % 10^(kappa+1)`)
    //
    //         // divide `remainder` by `10^kappa`. both are scaled by `2^-e`.
    //         let q = remainder / ten_kappa;
    //         let r = remainder % ten_kappa;
    //         debug_assert!(q < 10);
    //         buf[i] = b'0' + q as u8;
    //         i += 1;
    //
    //         let plus1rem = ((r as u64) << e) + plus1frac; // == (plus1 % 10^kappa) * 2^e
    //         if plus1rem < delta1 {
    //             // `plus1 % 10^kappa < delta1 = plus1 - minus1`; we've found the correct `kappa`.
    //             let ten_kappa = (ten_kappa as u64) << e; // scale 10^kappa back to the shared exponent
    //             return round_and_weed(&mut buf[..i], exp, plus1rem, delta1, plus1 - v.f, ten_kappa, 1);
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
    //     // render fractional parts, while checking for the accuracy at each step.
    //     // this time we rely on repeated multiplications, as division will lose the precision.
    //     let mut remainder = plus1frac;
    //     let mut threshold = delta1frac;
    //     let mut ulp = 1;
    //     loop { // the next digit should be significant as we've tested that before breaking out
    //         // invariants, where `m = max_kappa + 1` (# of digits in the integral part):
    //         // - `remainder < 2^e`
    //         // - `plus1frac * 10^(n-m) = d[m..n-1] * 2^e + remainder`
    //
    //         remainder *= 10; // won't overflow, `2^e * 10 < 2^64`
    //         threshold *= 10;
    //         ulp *= 10;
    //
    //         // divide `remainder` by `10^kappa`.
    //         // both are scaled by `2^e / 10^kappa`, so the latter is implicit here.
    //         let q = remainder >> e;
    //         let r = remainder & ((1 << e) - 1);
    //         debug_assert!(q < 10);
    //         buf[i] = b'0' + q as u8;
    //         i += 1;
    //
    //         if r < threshold {
    //             let ten_kappa = 1 << e; // implicit divisor
    //             return round_and_weed(&mut buf[..i], exp, r, threshold,
    //                                   (plus1 - v.f) * ulp, ten_kappa, ulp);
    //         }
    //
    //         // restore invariants
    //         kappa -= 1;
    //         remainder = r;
    //     }
    //
    //     // we've generated all significant digits of `plus1`, but not sure if it's the optimal one.
    //     // for example, if `minus1` is 3.14153... and `plus1` is 3.14158..., there are 5 different
    //     // shortest representation from 3.14154 to 3.14158 but we only have the greatest one.
    //     // we have to successively decrease the last digit and check if this is the optimal repr.
    //     // there are at most 9 candidates (..1 to ..9), so this is fairly quick. ("rounding" phase)
    //     //
    //     // the function checks if this "optimal" repr is actually within the ulp ranges,
    //     // and also, it is possible that the "second-to-optimal" repr can actually be optimal
    //     // due to the rounding error. in either cases this returns `None`. ("weeding" phase)
    //     //
    //     // all arguments here are scaled by the common (but implicit) value `k`, so that:
    //     // - `remainder = (plus1 % 10^kappa) * k`
    //     // - `threshold = (plus1 - minus1) * k` (and also, `remainder < threshold`)
    //     // - `plus1v = (plus1 - v) * k` (and also, `threshold > plus1v` from prior invariants)
    //     // - `ten_kappa = 10^kappa * k`
    //     // - `ulp = 2^-e * k`
    //     fn round_and_weed(buf: &mut [u8], exp: i16, remainder: u64, threshold: u64, plus1v: u64,
    //                       ten_kappa: u64, ulp: u64) -> Option<(usize, i16)> {
    //         assert!(!buf.is_empty());
    //
    //         // produce two approximations to `v` (actually `plus1 - v`) within 1.5 ulps.
    //         // the resulting representation should be the closest representation to both.
    //         //
    //         // here `plus1 - v` is used since calculations are done with respect to `plus1`
    //         // in order to avoid overflow/underflow (hence the seemingly swapped names).
    //         let plus1v_down = plus1v + ulp; // plus1 - (v - 1 ulp)
    //         let plus1v_up = plus1v - ulp; // plus1 - (v + 1 ulp)
    //
    //         // decrease the last digit and stop at the closest representation to `v + 1 ulp`.
    //         let mut plus1w = remainder; // plus1w(n) = plus1 - w(n)
    //         {
    //             let last = buf.last_mut().unwrap();
    //
    //             // we work with the approximated digits `w(n)`, which is initially equal to `plus1 -
    //             // plus1 % 10^kappa`. after running the loop body `n` times, `w(n) = plus1 -
    //             // plus1 % 10^kappa - n * 10^kappa`. we set `plus1w(n) = plus1 - w(n) =
    //             // plus1 % 10^kappa + n * 10^kappa` (thus `remainder = plus1w(0)`) to simplify checks.
    //             // note that `plus1w(n)` is always increasing.
    //             //
    //             // we have three conditions to terminate. any of them will make the loop unable to
    //             // proceed, but we then have at least one valid representation known to be closest to
    //             // `v + 1 ulp` anyway. we will denote them as TC1 through TC3 for brevity.
    //             //
    //             // TC1: `w(n) <= v + 1 ulp`, i.e. this is the last repr that can be the closest one.
    //             // this is equivalent to `plus1 - w(n) = plus1w(n) >= plus1 - (v + 1 ulp) = plus1v_up`.
    //             // combined with TC2 (which checks if `w(n+1)` is valid), this prevents the possible
    //             // overflow on the calculation of `plus1w(n)`.
    //             //
    //             // TC2: `w(n+1) < minus1`, i.e. the next repr definitely does not round to `v`.
    //             // this is equivalent to `plus1 - w(n) + 10^kappa = plus1w(n) + 10^kappa >
    //             // plus1 - minus1 = threshold`. the left hand side can overflow, but we know
    //             // `threshold > plus1v`, so if TC1 is false, `threshold - plus1w(n) >
    //             // threshold - (plus1v - 1 ulp) > 1 ulp` and we can safely test if
    //             // `threshold - plus1w(n) < 10^kappa` instead.
    //             //
    //             // TC3: `abs(w(n) - (v + 1 ulp)) <= abs(w(n+1) - (v + 1 ulp))`, i.e. the next repr is
    //             // no closer to `v + 1 ulp` than the current repr. given `z(n) = plus1v_up - plus1w(n)`,
    //             // this becomes `abs(z(n)) <= abs(z(n+1))`. again assuming that TC1 is false, we have
    //             // `z(n) > 0`. we have two cases to consider:
    //             //
    //             // - when `z(n+1) >= 0`: TC3 becomes `z(n) <= z(n+1)`. as `plus1w(n)` is increasing,
    //             //   `z(n)` should be decreasing and this is clearly false.
    //             // - when `z(n+1) < 0`:
    //             //   - TC3a: the precondition is `plus1v_up < plus1w(n) + 10^kappa`. assuming TC2 is
    //             //     false, `threshold >= plus1w(n) + 10^kappa` so it cannot overflow.
    //             //   - TC3b: TC3 becomes `z(n) <= -z(n+1)`, i.e. `plus1v_up - plus1w(n) >=
    //             //     plus1w(n+1) - plus1v_up = plus1w(n) + 10^kappa - plus1v_up`. the negated TC1
    //             //     gives `plus1v_up > plus1w(n)`, so it cannot overflow or underflow when
    //             //     combined with TC3a.
    //             //
    //             // consequently, we should stop when `TC1 || TC2 || (TC3a && TC3b)`. the following is
    //             // equal to its inverse, `!TC1 && !TC2 && (!TC3a || !TC3b)`.
    //             while plus1w < plus1v_up &&
    //                   threshold - plus1w >= ten_kappa &&
    //                   (plus1w + ten_kappa < plus1v_up ||
    //                    plus1v_up - plus1w >= plus1w + ten_kappa - plus1v_up) {
    //                 *last -= 1;
    //                 debug_assert!(*last > b'0'); // the shortest repr cannot end with `0`
    //                 plus1w += ten_kappa;
    //             }
    //         }
    //
    //         // check if this representation is also the closest representation to `v - 1 ulp`.
    //         //
    //         // this is simply same to the terminating conditions for `v + 1 ulp`, with all `plus1v_up`
    //         // replaced by `plus1v_down` instead. overflow analysis equally holds.
    //         if plus1w < plus1v_down &&
    //            threshold - plus1w >= ten_kappa &&
    //            (plus1w + ten_kappa < plus1v_down ||
    //             plus1v_down - plus1w >= plus1w + ten_kappa - plus1v_down) {
    //             return None;
    //         }
    //
    //         // now we have the closest representation to `v` between `plus1` and `minus1`.
    //         // this is too liberal, though, so we reject any `w(n)` not between `plus0` and `minus0`,
    //         // i.e. `plus1 - plus1w(n) <= minus0` or `plus1 - plus1w(n) >= plus0`. we utilize the facts
    //         // that `threshold = plus1 - minus1` and `plus1 - plus0 = minus0 - minus1 = 2 ulp`.
    //         if 2 * ulp <= plus1w && plus1w <= threshold - 4 * ulp {
    //             Some((buf.len(), exp))
    //         } else {
    //             None
    //         }
    //     }
    // }

    // pub fn format_shortest(d: &Decoded, buf: &mut [u8]) -> (/*#digits*/ usize, /*exp*/ i16) {
    //     use num::flt2dec::strategy::dragon::format_shortest as fallback;
    //     match format_shortest_opt(d, buf) {
    //         Some(ret) => ret,
    //         None => fallback(d, buf),
    //     }
    // }

    type T = f32; // T: DecodableFloat

    #[test]
    fn format_shortest_test1() {
	let v: T = 3.141592654;
	if let (_, Finite(d)) = decode::<T>(v) {
	    let mut buf: [u8; MAX_SIG_DIGITS] = [
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0
	    ];
	    let (digits, exp): (usize, i16) = format_shortest(&d, &mut buf);

	    assert_eq!(digits, 8);
	    assert_eq!(exp, 1);
	    assert_eq!(buf, [
		b'3', b'1', b'4', b'1', b'5', b'9', b'2', b'7', 0, 0,
		   0,    0,    0,    0,    0,    0,    0
	    ]);
	} else {
	    assert!(false);
	}
    }

    #[test]
    fn format_shortest_test2() {
	let v: T = 6.022e23;
	if let (_, Finite(d)) = decode::<T>(v) {
	    let mut buf: [u8; MAX_SIG_DIGITS] = [
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0
	    ];
	    let (digits, exp): (usize, i16) = format_shortest(&d, &mut buf);

	    assert_eq!(digits, 4);
	    assert_eq!(exp, 24);
	    assert_eq!(buf, [
		b'6', b'0', b'2', b'2', 0, 0, 0, 0, 0, 0,
		   0,    0,    0,    0, 0, 0, 0
	    ]);
	} else {
	    assert!(false);
	}
    }
}
