#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::return_address;

    // pub fn return_address() -> *const u8;

    #[allow(dead_code)]
    struct Point {
	x: f32,
	y: f32,
	z: f32
    }

    fn f(result: &mut usize) -> Point {
	let return_address: *const u8 = unsafe { return_address() };

	*result = return_address as usize;

	Point { x: 1.0, y: 2.0, z: 3.0 }
    }

    #[test]
    fn return_address_test1() {
	let mut core_intrinsic_reported_address: usize = 0;
	let pt: Point = f(&mut core_intrinsic_reported_address);
	let actual_address = &pt as *const Point as usize;

	assert_eq!(core_intrinsic_reported_address, actual_address);
    }
}
