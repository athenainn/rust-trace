#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::ceilf64;

    // pub fn ceilf64(x: f64) -> f64;

    #[test]
    fn ceilf64_test1() {
	let x: f64 = 0.123;
	let result: f64 = unsafe { ceilf64(x) };

	assert_eq!(result, 1.0);
    }

    #[test]
    fn ceilf64_test2() {
	let x: f64 = -0.123;
	let result: f64 = unsafe { ceilf64(x) };

	assert_eq!(result, -0.0);
    }
}
