#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    // pub fn size_of<T>() -> usize {
    //     unsafe { intrinsics::size_of::<T>() }
    // }

    #[test]
    fn size_of_test1() {
	struct A;

	let size: usize = size_of::<A>();

	assert_eq!(size, 0);
    }

    #[test]
    fn size_of_test2() {
	assert_eq!(size_of::<u8>(), 1);
	assert_eq!(size_of::<i8>(), 1);
	assert_eq!(size_of::<u16>(), 2);
	assert_eq!(size_of::<i16>(), 2);
	assert_eq!(size_of::<u32>(), 4);
	assert_eq!(size_of::<i32>(), 4);
	assert_eq!(size_of::<u64>(), 8);
	assert_eq!(size_of::<i64>(), 8);
	assert_eq!(size_of::<isize>(), 8);
	assert_eq!(size_of::<usize>(), 8);
	assert_eq!(size_of::<f32>(), 4);
	assert_eq!(size_of::<f64>(), 8);
    }
}
