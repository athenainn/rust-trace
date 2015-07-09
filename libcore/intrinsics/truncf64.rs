#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::truncf64;

    // pub fn truncf64(x: f64) -> f64;

    #[test]
    fn truncf64_test1() {
	let x: f64 = 12345.67890;
	let result: f64 = unsafe { truncf64(x) };

	assert_eq!(result, 12345.0);
    }
}
