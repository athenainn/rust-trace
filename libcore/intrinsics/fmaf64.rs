#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::fmaf64;

    // pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;

    #[test]
    fn fmaf64_test1() {
	let a: f64 = 68.0;
	let b: f64 = 100.0;
	let c: f64 = 1.0;
	let result: f64 = unsafe { fmaf64(a, b, c) };

	assert_eq!(result, 6801.0);
    }
}
