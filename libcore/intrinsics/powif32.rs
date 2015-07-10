#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::powif32;

    use core::num::Float;

    use core::f32;

    // pub fn powif32(a: f32, x: i32) -> f32;

    #[test]
    fn powif32_test1() {
	let a: f32 = f32::nan();
	let x: i32 = 0;
	let result: f32 = unsafe { powif32(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powif32_test2() {
	let a: f32 = f32::nan();
	let x: i32 = 1; // non-zero
	let result: f32 = unsafe { powif32(a, x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn powif32_test3() {
	let a: f32 = f32::infinity();
	let x: i32 = 0;
	let result: f32 = unsafe { powif32(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powif32_test4() {
	let a: f32 = f32::infinity();
	let x: i32 = 1; // non-zero
	let result: f32 = unsafe { powif32(a, x) };

	assert_eq!(result, f32::infinity());
    }

    #[test]
    fn powif32_test5() {
	let a: f32 = f32::neg_infinity();
	let x: i32 = 0;
	let result: f32 = unsafe { powif32(a, x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn powif32_test6() {
	let a: f32 = f32::neg_infinity();
	let x: i32 = 1; // non-zero
	let result: f32 = unsafe { powif32(a, x) };

	assert_eq!(result, f32::neg_infinity());
    }

    #[test]
    fn powif32_test7() {
	let a: f32 = 2.0;
	let x: i32 = 10;
	let result: f32 = unsafe { powif32(a, x) };

	assert_eq!(result, 1024.0);
    }
}
