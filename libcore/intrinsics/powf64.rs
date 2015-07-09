#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::powf64;

    use core::num::Float;

    use core::f64;

    // pub fn powf64(a: f64, x: f64) -> f64;

    #[test]
    fn powf64_test1() {
	let a: f64 = f64::nan();
	let x: f64 = 0.0;
	let result: f64 = unsafe { powf64(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powf64_test2() {
	let a: f64 = f64::nan();
	let x: f64 = 1.0; // non-zero
	let result: f64 = unsafe { powf64(a, x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn powf64_test3() {
	let a: f64 = f64::infinity();
	let x: f64 = 0.0;
	let result: f64 = unsafe { powf64(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powf64_test4() {
	let a: f64 = f64::infinity();
	let x: f64 = 1.0; // non-zero
	let result: f64 = unsafe { powf64(a, x) };

	assert_eq!(result, f64::infinity());
    }

    #[test]
    fn powf64_test5() {
	let a: f64 = f64::neg_infinity();
	let x: f64 = 0.0;
	let result: f64 = unsafe { powf64(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powf64_test6() {
	let a: f64 = f64::neg_infinity();
	let x: f64 = 1.0; // non-zero
	let result: f64 = unsafe { powf64(a, x) };

	assert_eq!(result, f64::neg_infinity());
    }

    #[test]
    fn powf64_test7() {
	let a: f64 = 2.0;
	let x: f64 = 10.0;
	let result: f64 = unsafe { powf64(a, x) };

	assert_eq!(result, 1024.0);
    }
}
