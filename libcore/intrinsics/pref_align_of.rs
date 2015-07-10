#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::pref_align_of;

    // pub fn pref_align_of<T>(dst: &mut T, src: T);

    macro_rules! pref_align_of_test {
	($T:ty, $size:expr) => ({
	    let size: usize = unsafe { pref_align_of::<$T>() };

	    assert_eq!(size, $size);
	})
    }

    #[test]
    fn pref_align_of_test1() {
	pref_align_of_test!( u8, 1 );
	pref_align_of_test!( u16, 2 );
	pref_align_of_test!( u32, 4 );
	pref_align_of_test!( u64, 8 );
	pref_align_of_test!( i8, 1 );
	pref_align_of_test!( i16, 2 );
	pref_align_of_test!( i32, 4 );
	pref_align_of_test!( i64, 8 );

	pref_align_of_test!( f32, 4 );
	pref_align_of_test!( f64, 8 );

	pref_align_of_test!( [u8; 0], 1 );
	pref_align_of_test!( [u8; 68], 1 );

	pref_align_of_test!( [u32; 0], 4 );
	pref_align_of_test!( [u32; 68], 4 );

	pref_align_of_test!( (u8,), 8 );
	pref_align_of_test!( (u8, u16), 8 );
	pref_align_of_test!( (u8, u16, u32), 8 );
	pref_align_of_test!( (u8, u16, u32, u64), 8 );
    }
}
