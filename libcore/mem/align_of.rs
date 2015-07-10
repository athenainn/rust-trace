#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::align_of;

    // pub fn align_of<T>() -> usize {
    //     unsafe { intrinsics::min_align_of::<T>() }
    // }

    macro_rules! align_of_test {
	($T:ty, $size:expr) => ({
	    let size: usize = align_of::<$T>();

	    assert_eq!(size, $size);
	})
    }

    #[test]
    fn align_of_test1() {
	struct A;

	align_of_test!( A, 1 );
    }

    #[test]
    fn align_of_test2() {
	align_of_test!( u8, 1 );
	align_of_test!( u16, 2 );
	align_of_test!( u32, 4 );
	align_of_test!( u64, 8 );
	align_of_test!( i8, 1 );
	align_of_test!( i16, 2 );
	align_of_test!( i32, 4 );
	align_of_test!( i64, 8 );

	align_of_test!( f32, 4 );
	align_of_test!( f64, 8 );

	align_of_test!( [u8; 0], 1 );
	align_of_test!( [u8; 68], 1 );

	align_of_test!( [u32; 0], 4 );
	align_of_test!( [u32; 68], 4 );

	align_of_test!( (u8,), 1 );
	align_of_test!( (u8, u16), 2 );
	align_of_test!( (u8, u16, u32), 4 );
	align_of_test!( (u8, u16, u32, u64), 8 );
    }
}
