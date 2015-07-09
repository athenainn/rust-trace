#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::cosf32;

    use core::num::Float;

    use core::f32;
    use core::f32::consts::PI;

    // pub fn cosf32(x: f32) -> f32;

    #[test]
    fn cosf32_test1() {
	let x: f32 = f32::nan();
	let result: f32 = unsafe { cosf32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn cosf32_test2() {
	let x: f32 = f32::infinity();
	let result: f32 = unsafe { cosf32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn cosf32_test3() {
	let x: f32 = f32::neg_infinity();
	let result: f32 = unsafe { cosf32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn cosf32_test4() {
	let x: f32 = 0.0 * PI / 180.0;
	let result: f32 = unsafe { cosf32(x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn cosf32_test5() {
	let x: f32 = 45.0 * PI / 180.0;
	let result: f32 = unsafe { cosf32(x) };

	assert_eq!(result, 0.70710677);
    }

    #[test]
    fn cosf32_test6() {
	let x: f32 = 90.0 * PI / 180.0;
	let result: f32 = unsafe { cosf32(x) };

	assert_eq!(result, -0.00000004371139);
    }
}
