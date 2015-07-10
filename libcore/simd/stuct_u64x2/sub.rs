#![feature(core, core_simd)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::simd::u64x2;

    // #[simd]
    // #[derive(Copy, Clone, Debug)]
    // #[repr(C)]
    // pub struct u64x2(pub u64, pub u64);

    #[test]
    fn sub_test1() {
	let x: u64x2 = u64x2(
	     0,  1
	);
	let y: u64x2 = u64x2(
	    2, 2
	);
	let z: u64x2 = x - y;

	let result: String = format!("{:?}", z);
	assert_eq!(result, "u64x2(\
	    18446744073709551614, 18446744073709551615\
	    )".to_string());
    }
}
