#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::expf64;

    use core::num::Float;

    use core::f64;

    // pub fn expf64(x: f64) -> f64;

    #[test]
    fn expf64_test1() {
	let x: f64 = f64::nan();
	let result: f64 = unsafe { expf64(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn expf64_test2() {
	let x: f64 = f64::infinity();
	let result: f64 = unsafe { expf64(x) };

	assert_eq!(result, f64::infinity());
    }

    #[test]
    fn expf64_test3() {
	let x: f64 = f64::neg_infinity();
	let result: f64 = unsafe { expf64(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn expf64_test4() {
	let x: f64 = 1.0;
	let result: f64 = unsafe { expf64(x) };

	assert_eq!(result, 2.718281828459045);
    }
}
