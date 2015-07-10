#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::min_align_of;

    // pub fn min_align_of<T>(dst: &mut T, src: T);

    macro_rules! min_align_of_test {
	($T:ty, $size:expr) => ({
	    let size: usize = unsafe { min_align_of::<$T>() };

	    assert_eq!(size, $size);
	})
    }

    #[test]
    fn min_align_of_test1() {
	struct A;

	min_align_of_test!( A, 1 );
    }

    #[test]
    fn min_align_of_test2() {
	min_align_of_test!( u8, 1 );
	min_align_of_test!( u16, 2 );
	min_align_of_test!( u32, 4 );
	min_align_of_test!( u64, 8 );
	min_align_of_test!( i8, 1 );
	min_align_of_test!( i16, 2 );
	min_align_of_test!( i32, 4 );
	min_align_of_test!( i64, 8 );

	min_align_of_test!( f32, 4 );
	min_align_of_test!( f64, 8 );

	min_align_of_test!( [u8; 0], 1 );
	min_align_of_test!( [u8; 68], 1 );

	min_align_of_test!( [u32; 0], 4 );
	min_align_of_test!( [u32; 68], 4 );

	min_align_of_test!( (u8,), 1 );
	min_align_of_test!( (u8, u16), 2 );
	min_align_of_test!( (u8, u16, u32), 4 );
	min_align_of_test!( (u8, u16, u32, u64), 8 );
    }
}
