#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::rintf64;

    // pub fn rintf64(x: f64) -> f64;

    // Round half to even:
    //
    // A tie-breaking rule that is less biased is round half to even, namely:
    //   * If the fraction of y is 0.5, then q is the even integer nearest to y.
    //
    // This is the default rounding mode used in IEEE 754 computing functions
    // and operators.
    //
    // Reference: https://en.wikipedia.org/wiki/Rounding#Round_half_to_even

    #[test]
    fn rintf64_test1() {
	let x: f64 = 23.5;
	let result: f64 = unsafe { rintf64(x) };

	assert_eq!(result, 24.0);
    }

    #[test]
    fn rintf64_test2() {
	let x: f64 = 24.5;
	let result: f64 = unsafe { rintf64(x) };

	assert_eq!(result, 24.0);
    }

    #[test]
    fn rintf64_test3() {
	let x: f64 = -23.5;
	let result: f64 = unsafe { rintf64(x) };

	assert_eq!(result, -24.0);
    }

    #[test]
    fn rintf64_test4() {
	let x: f64 = -24.5;
	let result: f64 = unsafe { rintf64(x) };

	assert_eq!(result, -24.0);
    }
}
