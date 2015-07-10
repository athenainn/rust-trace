#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::powf32;

    use core::num::Float;

    use core::f32;

    // pub fn powf32(a: f32, x: f32) -> f32;

    #[test]
    fn powf32_test1() {
	let a: f32 = f32::nan();
	let x: f32 = 0.0;
	let result: f32 = unsafe { powf32(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powf32_test2() {
	let a: f32 = f32::nan();
	let x: f32 = 1.0; // non-zero
	let result: f32 = unsafe { powf32(a, x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn powf32_test3() {
	let a: f32 = f32::infinity();
	let x: f32 = 0.0;
	let result: f32 = unsafe { powf32(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powf32_test4() {
	let a: f32 = f32::infinity();
	let x: f32 = 1.0; // non-zero
	let result: f32 = unsafe { powf32(a, x) };

	assert_eq!(result, f32::infinity());
    }

    #[test]
    fn powf32_test5() {
	let a: f32 = f32::neg_infinity();
	let x: f32 = 0.0;
	let result: f32 = unsafe { powf32(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powf32_test6() {
	let a: f32 = f32::neg_infinity();
	let x: f32 = 1.0; // non-zero
	let result: f32 = unsafe { powf32(a, x) };

	assert_eq!(result, f32::neg_infinity());
    }

    #[test]
    fn powf32_test7() {
	let a: f32 = 2.0;
	let x: f32 = 10.0;
	let result: f32 = unsafe { powf32(a, x) };

	assert_eq!(result, 1024.0);
    }
}
