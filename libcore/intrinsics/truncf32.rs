#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::truncf32;

    // pub fn truncf32(x: f32) -> f32;

    #[test]
    fn truncf32_test1() {
	let x: f32 = 12345.67890;
	let result: f32 = unsafe { truncf32(x) };

	assert_eq!(result, 12345.0);
    }
}
