#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::sqrtf64;

    use core::num::Float;

    use core::f64;

    // pub fn sqrtf64(x: f64) -> f64;

    #[test]
    fn sqrtf64_test1() {
	let x: f64 = f64::nan();
	let result: f64 = unsafe { sqrtf64(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn sqrtf64_test2() {
	let x: f64 = f64::infinity();
	let result: f64 = unsafe { sqrtf64(x) };

	assert_eq!(result, f64::infinity());
    }

    #[test]
    fn sqrtf64_test3() {
	let x: f64 = f64::neg_infinity();
	let result: f64 = unsafe { sqrtf64(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn sqrtf64_test4() {
	let x: f64 = f64::zero();
	let result: f64 = unsafe { sqrtf64(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn sqrtf64_test5() {
	let x: f64 = f64::one();
	let result: f64 = unsafe { sqrtf64(x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn sqrtf64_test6() {
	let x: f64 = 2.0;
	let result: f64 = unsafe { sqrtf64(x) };

	assert_eq!(result, 1.4142135623730951);
    }
}
