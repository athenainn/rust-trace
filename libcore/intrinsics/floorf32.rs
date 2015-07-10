#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::floorf32;

    // pub fn floorf32(x: f32) -> f32;

    #[test]
    fn floorf32_test1() {
	let x: f32 = 0.123;
	let result: f32 = unsafe { floorf32(x) };

	assert_eq!(result, 0.0);
    }

    #[test]
    fn floorf32_test2() {
	let x: f32 = -0.123;
	let result: f32 = unsafe { floorf32(x) };

	assert_eq!(result, -1.0);
    }
}
