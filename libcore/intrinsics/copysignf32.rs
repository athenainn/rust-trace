#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::copysignf32;

    // pub fn copysignf32(x: f32, y: f32) -> f32;

    #[test]
    fn copysignf32_test1() {
	let x: f32 = 68.0;
	let y: f32 = -500.0;
	let result: f32 = unsafe { copysignf32(x, y) };

	assert_eq!(result, -68.0);
    }

    #[test]
    fn copysignf32_test2() {
	let x: f32 = -68.0;
	let y: f32 = 500.0;
	let result: f32 = unsafe { copysignf32(x, y) };

	assert_eq!(result, 68.0);
    }
}
