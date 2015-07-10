#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::logf32;

    use core::num::Float;

    use core::f32;
    use core::f32::consts::E;

    // pub fn logf32(x: f32) -> f32;

    #[test]
    fn logf32_test1() {
	let x: f32 = f32::nan();
	let result: f32 = unsafe { logf32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn logf32_test2() {
	let x: f32 = f32::infinity();
	let result: f32 = unsafe { logf32(x) };

	assert_eq!(result, f32::infinity());
    }

    #[test]
    fn logf32_test3() {
	let x: f32 = f32::neg_infinity();
	let result: f32 = unsafe { logf32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn logf32_test4() {
	let x: f32 = 1.0;
	let result: f32 = unsafe { logf32(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn logf32_test5() {
	let x: f32 = E;
	let result: f32 = unsafe { logf32(x) };

	assert_eq!(result, 0.99999994);
    }
}
