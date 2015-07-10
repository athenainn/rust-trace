#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::floorf64;

    // pub fn floorf64(x: f64) -> f64;

    #[test]
    fn floorf64_test1() {
	let x: f64 = 0.123;
	let result: f64 = unsafe { floorf64(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn floorf64_test2() {
	let x: f64 = -0.123;
	let result: f64 = unsafe { floorf64(x) };

	assert_eq!(result, -1.0);
    }
}
