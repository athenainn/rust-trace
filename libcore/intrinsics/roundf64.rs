#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::roundf64;

    // pub fn roundf64(x: f64) -> f64;

    // Round half away from zero:
    //
    // The other tie-breaking method commonly taught and used is the round half
    // away from zero (or round half towards infinity), namely:
    //   * If the fraction of y is exactly 0.5, then q = y + 0.5 if y is
    //     positive, and q = y − 0.5 if y is negative.
    //
    // Reference: https://en.wikipedia.org/wiki/Rounding#Round_half_away_from_zero

    #[test]
    fn roundf64_test1() {
	let x: f64 = 23.5;
	let result: f64 = unsafe { roundf64(x) };

	assert_eq!(result, 24.0);
    }

    #[test]
    fn roundf64_test2() {
	let x: f64 = 24.5;
	let result: f64 = unsafe { roundf64(x) };

	assert_eq!(result, 25.0);
    }

    #[test]
    fn roundf64_test3() {
	let x: f64 = -23.5;
	let result: f64 = unsafe { roundf64(x) };

	assert_eq!(result, -24.0);
    }

    #[test]
    fn roundf64_test4() {
	let x: f64 = -24.5;
	let result: f64 = unsafe { roundf64(x) };

	assert_eq!(result, -25.0);
    }
}
