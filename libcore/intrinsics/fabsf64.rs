#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::fabsf64;

    use core::num::Float;

    use core::f64;

    // pub fn fabsf64(x: f64) -> f64;

    #[test]
    fn fabsf64_test1() {
	let x: f64 = f64::nan();
	let result: f64 = unsafe { fabsf64(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn fabsf64_test2() {
	let x: f64 = f64::infinity();
	let result: f64 = unsafe { fabsf64(x) };

	assert_eq!(result, f64::infinity());
    }

    #[test]
    fn fabsf64_test3() {
	let x: f64 = f64::neg_infinity();
	let result: f64 = unsafe { fabsf64(x) };

	assert_eq!(result, f64::infinity());
    }

    #[test]
    fn fabsf64_test4() {
	let x: f64 = -68.0;
	let result: f64 = unsafe { fabsf64(x) };

	assert_eq!(result, 68.0);
    }
}
