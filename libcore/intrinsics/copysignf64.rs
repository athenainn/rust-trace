#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::copysignf64;

    // pub fn copysignf64(x: f64, y: f64) -> f64;

    #[test]
    fn copysignf64_test1() {
	let x: f64 = 68.0;
	let y: f64 = -500.0;
	let result: f64 = unsafe { copysignf64(x, y) };

	assert_eq!(result, -68.0);
    }

    #[test]
    fn copysignf64_test2() {
	let x: f64 = -68.0;
	let y: f64 = 500.0;
	let result: f64 = unsafe { copysignf64(x, y) };

	assert_eq!(result, 68.0);
    }
}
