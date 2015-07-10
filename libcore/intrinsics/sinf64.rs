#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::sinf64;

    use core::num::Float;

    use core::f64;
    use core::f64::consts::PI;

    // pub fn sinf64(x: f64) -> f64;

    #[test]
    fn sinf64_test1() {
	let x: f64 = f64::nan();
	let result: f64 = unsafe { sinf64(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn sinf64_test2() {
	let x: f64 = f64::infinity();
	let result: f64 = unsafe { sinf64(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn sinf64_test3() {
	let x: f64 = f64::neg_infinity();
	let result: f64 = unsafe { sinf64(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn sinf64_test4() {
	let x: f64 = 0.0 * PI / 180.0;
	let result: f64 = unsafe { sinf64(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn sinf64_test5() {
	let x: f64 = 45.0 * PI / 180.0;
	let result: f64 = unsafe { sinf64(x) };

	assert_eq!(result, 0.7071067811865475);
    }

    #[test]
    fn sinf64_test6() {
	let x: f64 = 90.0 * PI / 180.0;
	let result: f64 = unsafe { sinf64(x) };

	assert_eq!(result, 1.0);
    }
}
