#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::size_of;

    // pub fn size_of<T>() -> usize;

    macro_rules! size_of_test {
	($T:ty, $size:expr) => ({
	    let size: usize = unsafe { size_of::<$T>() };

	    assert_eq!(size, $size);
	})
    }

    #[test]
    fn size_of_test1() {
	struct A;

	size_of_test!( A, 0 );
    }

    #[test]
    fn size_of_test2() {
	size_of_test!( u8, 1 );
	size_of_test!( u16, 2 );
	size_of_test!( u32, 4 );
	size_of_test!( u64, 8 );
	size_of_test!( i8, 1 );
	size_of_test!( i16, 2 );
	size_of_test!( i32, 4 );
	size_of_test!( i64, 8 );

	size_of_test!( f32, 4 );
	size_of_test!( f64, 8 );

	size_of_test!( [u8; 0], 0 );
	size_of_test!( [u8; 68], 68 );

	size_of_test!( [u32; 0], 0 );
	size_of_test!( [u32; 68], 4 * 68 );

	size_of_test!( (u8,), 1 );
	size_of_test!( (u8, u16), 4 );
	size_of_test!( (u8, u16, u32), 8 );
	size_of_test!( (u8, u16, u32, u64), 16 );
    }
}
