#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::fmaf32;

    // pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;

    #[test]
    fn fmaf32_test1() {
	let a: f32 = 68.0;
	let b: f32 = 100.0;
	let c: f32 = 1.0;
	let result: f32 = unsafe { fmaf32(a, b, c) };

	assert_eq!(result, 6801.0);
    }
}
