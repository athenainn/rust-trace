#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::sqrtf32;

    use core::num::Float;

    use core::f32;

    // pub fn sqrtf32(x: f32) -> f32;

    #[test]
    fn sqrtf32_test1() {
	let x: f32 = f32::nan();
	let result: f32 = unsafe { sqrtf32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn sqrtf32_test2() {
	let x: f32 = f32::infinity();
	let result: f32 = unsafe { sqrtf32(x) };

	assert_eq!(result, f32::infinity());
    }

    #[test]
    fn sqrtf32_test3() {
	let x: f32 = f32::neg_infinity();
	let result: f32 = unsafe { sqrtf32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn sqrtf32_test4() {
	let x: f32 = f32::zero();
	let result: f32 = unsafe { sqrtf32(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn sqrtf32_test5() {
	let x: f32 = f32::one();
	let result: f32 = unsafe { sqrtf32(x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn sqrtf32_test6() {
	let x: f32 = 2.0;
	let result: f32 = unsafe { sqrtf32(x) };

	assert_eq!(result, 1.4142135);
    }
}
