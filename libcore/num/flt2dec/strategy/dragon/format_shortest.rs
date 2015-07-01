#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::MAX_SIG_DIGITS;
    use core::num::flt2dec::strategy::dragon::format_shortest;

    use core::num::flt2dec::FullDecoded::{self, Finite};
    use core::num::flt2dec::decode;

    // pub fn format_shortest(d: &Decoded, buf: &mut [u8]) -> (/*#digits*/ usize, /*exp*/ i16) {
    //     // the number `v` to format is known to be:
    //     // - equal to `mant * 2^exp`;
    //     // - preceded by `(mant - 2 * minus) * 2^exp` in the original type; and
    //     // - followed by `(mant + 2 * plus) * 2^exp` in the original type.
    //     //
    //     // obviously, `minus` and `plus` cannot be zero. (for infinities, we use out-of-range values.)
    //     // also we assume that at least one digit is generated, i.e. `mant` cannot be zero too.
    //     //
    //     // this also means that any number between `low = (mant - minus) * 2^exp` and
    //     // `high = (mant + plus) * 2^exp` will map to this exact floating point number,
    //     // with bounds included when the original mantissa was even (i.e. `!mant_was_odd`).
    //
    //     assert!(d.mant > 0);
    //     assert!(d.minus > 0);
    //     assert!(d.plus > 0);
    //     assert!(d.mant.checked_add(d.plus).is_some());
    //     assert!(d.mant.checked_sub(d.minus).is_some());
    //     assert!(buf.len() >= MAX_SIG_DIGITS);
    //
    //     // `a.cmp(&b) < rounding` is `if d.inclusive {a <= b} else {a < b}`
    //     let rounding = if d.inclusive {Ordering::Greater} else {Ordering::Equal};
    //
    //     // estimate `k_0` from original inputs satisfying `10^(k_0-1) < high <= 10^(k_0+1)`.
    //     // the tight bound `k` satisfying `10^(k-1) < high <= 10^k` is calculated later.
    //     let mut k = estimate_scaling_factor(d.mant + d.plus, d.exp);
    //
    //     // convert `{mant, plus, minus} * 2^exp` into the fractional form so that:
    //     // - `v = mant / scale`
    //     // - `low = (mant - minus) / scale`
    //     // - `high = (mant + plus) / scale`
    //     let mut mant = Big::from_u64(d.mant);
    //     let mut minus = Big::from_u64(d.minus);
    //     let mut plus = Big::from_u64(d.plus);
    //     let mut scale = Big::from_small(1);
    //     if d.exp < 0 {
    //         scale.mul_pow2(-d.exp as usize);
    //     } else {
    //         mant.mul_pow2(d.exp as usize);
    //         minus.mul_pow2(d.exp as usize);
    //         plus.mul_pow2(d.exp as usize);
    //     }
    //
    //     // divide `mant` by `10^k`. now `scale / 10 < mant + plus <= scale * 10`.
    //     if k >= 0 {
    //         mul_pow10(&mut scale, k as usize);
    //     } else {
    //         mul_pow10(&mut mant, -k as usize);
    //         mul_pow10(&mut minus, -k as usize);
    //         mul_pow10(&mut plus, -k as usize);
    //     }
    //
    //     // fixup when `mant + plus > scale` (or `>=`).
    //     // we are not actually modifying `scale`, since we can skip the initial multiplication instead.
    //     // now `scale < mant + plus <= scale * 10` and we are ready to generate digits.
    //     //
    //     // note that `d[0]` *can* be zero, when `scale - plus < mant < scale`.
    //     // in this case rounding-up condition (`up` below) will be triggered immediately.
    //     if scale.cmp(mant.clone().add(&plus)) < rounding {
    //         // equivalent to scaling `scale` by 10
    //         k += 1;
    //     } else {
    //         mant.mul_small(10);
    //         minus.mul_small(10);
    //         plus.mul_small(10);
    //     }
    //
    //     // cache `(2, 4, 8) * scale` for digit generation.
    //     let mut scale2 = scale.clone(); scale2.mul_pow2(1);
    //     let mut scale4 = scale.clone(); scale4.mul_pow2(2);
    //     let mut scale8 = scale.clone(); scale8.mul_pow2(3);
    //
    //     let mut down;
    //     let mut up;
    //     let mut i = 0;
    //     loop {
    //         // invariants, where `d[0..n-1]` are digits generated so far:
    //         // - `v = mant / scale * 10^(k-n-1) + d[0..n-1] * 10^(k-n)`
    //         // - `v - low = minus / scale * 10^(k-n-1)`
    //         // - `high - v = plus / scale * 10^(k-n-1)`
    //         // - `(mant + plus) / scale <= 10` (thus `mant / scale < 10`)
    //         // where `d[i..j]` is a shorthand for `d[i] * 10^(j-i) + ... + d[j-1] * 10 + d[j]`.
    //
    //         // generate one digit: `d[n] = floor(mant / scale) < 10`.
    //         let (d, _) = div_rem_upto_16(&mut mant, &scale, &scale2, &scale4, &scale8);
    //         debug_assert!(d < 10);
    //         buf[i] = b'0' + d;
    //         i += 1;
    //
    //         // this is a simplified description of the modified Dragon algorithm.
    //         // many intermediate derivations and completeness arguments are omitted for convenience.
    //         //
    //         // start with modified invariants, as we've updated `n`:
    //         // - `v = mant / scale * 10^(k-n) + d[0..n-1] * 10^(k-n)`
    //         // - `v - low = minus / scale * 10^(k-n)`
    //         // - `high - v = plus / scale * 10^(k-n)`
    //         //
    //         // assume that `d[0..n-1]` is the shortest representation between `low` and `high`,
    //         // i.e. `d[0..n-1]` satisfies both of the following but `d[0..n-2]` doesn't:
    //         // - `low < d[0..n-1] * 10^(k-n) < high` (bijectivity: digits round to `v`); and
    //         // - `abs(v / 10^(k-n) - d[0..n-1]) <= 1/2` (the last digit is correct).
    //         //
    //         // the second condition simplifies to `2 * mant <= scale`.
    //         // solving invariants in terms of `mant`, `low` and `high` yields
    //         // a simpler version of the first condition: `-plus < mant < minus`.
    //         // since `-plus < 0 <= mant`, we have the correct shortest representation
    //         // when `mant < minus` and `2 * mant <= scale`.
    //         // (the former becomes `mant <= minus` when the original mantissa is even.)
    //         //
    //         // when the second doesn't hold (`2 * mant > scale`), we need to increase the last digit.
    //         // this is enough for restoring that condition: we already know that
    //         // the digit generation guarantees `0 <= v / 10^(k-n) - d[0..n-1] < 1`.
    //         // in this case, the first condition becomes `-plus < mant - scale < minus`.
    //         // since `mant < scale` after the generation, we have `scale < mant + plus`.
    //         // (again, this becomes `scale <= mant + plus` when the original mantissa is even.)
    //         //
    //         // in short:
    //         // - stop and round `down` (keep digits as is) when `mant < minus` (or `<=`).
    //         // - stop and round `up` (increase the last digit) when `scale < mant + plus` (or `<=`).
    //         // - keep generating otherwise.
    //         down = mant.cmp(&minus) < rounding;
    //         up = scale.cmp(mant.clone().add(&plus)) < rounding;
    //         if down || up { break; } // we have the shortest representation, proceed to the rounding
    //
    //         // restore the invariants.
    //         // this makes the algorithm always terminating: `minus` and `plus` always increases,
    //         // but `mant` is clipped modulo `scale` and `scale` is fixed.
    //         mant.mul_small(10);
    //         minus.mul_small(10);
    //         plus.mul_small(10);
    //     }
    //
    //     // rounding up happens when
    //     // i) only the rounding-up condition was triggered, or
    //     // ii) both conditions were triggered and tie breaking prefers rounding up.
    //     if up && (!down || *mant.mul_pow2(1) >= scale) {
    //         // if rounding up changes the length, the exponent should also change.
    //         // it seems that this condition is very hard to satisfy (possibly impossible),
    //         // but we are just being safe and consistent here.
    //         if let Some(c) = round_up(buf, i) {
    //             buf[i] = c;
    //             i += 1;
    //             k += 1;
    //         }
    //     }
    //
    //     (i, k)
    // }

    type T = f32; // T: DecodableFloat

    #[test]
    fn format_shortest_test1() {
	let v: T = 3.141592654;
	let (_, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	match fulldecoded {
	    Finite(d) => {
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
	    }
	    _ => assert!(false)
	}
    }

    #[test]
    fn format_shortest_test2() {
	let v: T = 6.022e23;
	let (_, fulldecoded): (bool, FullDecoded) = decode::<T>(v);

	match fulldecoded {
	    Finite(d) => {
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
	    }
	    _ => assert!(false)
	}
    }
}
