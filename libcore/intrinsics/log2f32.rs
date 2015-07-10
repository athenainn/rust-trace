#![feature(core, core_intrinsics, core_float)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::log2f32;

    use core::num::Float;

    use core::f32;

    // pub fn log2f32(x: f32) -> f32;

    #[test]
    fn log2f32_test1() {
	let x: f32 = f32::nan();
	let result: f32 = unsafe { log2f32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn log2f32_test2() {
	let x: f32 = f32::infinity();
	let result: f32 = unsafe { log2f32(x) };

	assert_eq!(result, f32::infinity());
    }

    #[test]
    fn log2f32_test3() {
	let x: f32 = f32::neg_infinity();
	let result: f32 = unsafe { log2f32(x) };

	assert_eq!(result.is_nan(), true);
    }

    #[test]
    fn log2f32_test4() {
	let x: f32 = 1.0;
	let result: f32 = unsafe { log2f32(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn log2f32_test5() {
	let x: f32 = 1024.0;
	let result: f32 = unsafe { log2f32(x) };

	assert_eq!(result, 10.0);
    }
}
