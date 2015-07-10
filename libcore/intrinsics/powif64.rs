#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::powif64;

    use core::num::Float;

    use core::f64;

    // pub fn powif64(a: f64, x: i64) -> f64;

    #[test]
    fn powif64_test1() {
	let a: f64 = f64::nan();
	let x: i32 = 0;
	let result: f64 = unsafe { powif64(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powif64_test2() {
	let a: f64 = f64::nan();
	let x: i32 = 1; // non-zero
	let result: f64 = unsafe { powif64(a, x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn powif64_test3() {
	let a: f64 = f64::infinity();
	let x: i32 = 0;
	let result: f64 = unsafe { powif64(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powif64_test4() {
	let a: f64 = f64::infinity();
	let x: i32 = 1; // non-zero
	let result: f64 = unsafe { powif64(a, x) };

	assert_eq!(result, f64::infinity());
    }

    #[test]
    fn powif64_test5() {
	let a: f64 = f64::neg_infinity();
	let x: i32 = 0;
	let result: f64 = unsafe { powif64(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powif64_test6() {
	let a: f64 = f64::neg_infinity();
	let x: i32 = 1; // non-zero
	let result: f64 = unsafe { powif64(a, x) };

	assert_eq!(result, f64::neg_infinity());
    }

    #[test]
    fn powif64_test7() {
	let a: f64 = 2.0;
	let x: i32 = 10;
	let result: f64 = unsafe { powif64(a, x) };

	assert_eq!(result, 1024.0);
    }
}
