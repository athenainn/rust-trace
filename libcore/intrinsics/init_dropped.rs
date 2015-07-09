#![feature(core, core_intrinsics, filling_drop)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::init_dropped;
    use core::intrinsics::forget;

    use core::mem::POST_DROP_U8;
    use core::mem::POST_DROP_U32;
    use core::mem::POST_DROP_U64;
    use core::mem::POST_DROP_USIZE;

    // pub fn init_dropped<T>() -> T;

    macro_rules! init_dropped_test {
	($T:ty, $value:expr) => ({
	    let value: $T = unsafe { init_dropped::<$T>() };

	    assert_eq!(value, $value);

	    unsafe { forget::<$T>(value); }
	})
    }

    #[test]
    fn init_dropped_test1() {
	init_dropped_test!( u8, POST_DROP_U8 );
	init_dropped_test!( u16, 0x1d1d );
	init_dropped_test!( u32, POST_DROP_U32 );
	init_dropped_test!( u64, POST_DROP_U64 );
	init_dropped_test!( usize, POST_DROP_USIZE );

	init_dropped_test!( i8, POST_DROP_U8 as i8 );
	init_dropped_test!( i16, 0x1d1d );
	init_dropped_test!( i32, POST_DROP_U32 as i32 );
	init_dropped_test!( i64, POST_DROP_U64 as i64 );
	init_dropped_test!( isize, POST_DROP_USIZE as isize );

	init_dropped_test!( f32, unsafe {
	    *(&0x1d1d1d1d_u32 as *const u32 as *const f32)
	} );
	init_dropped_test!( f64, unsafe {
	    *(&0x1d1d1d1d1d1d1d1d_u64 as *const u64 as *const f64)
	} );

	init_dropped_test!( [u8; 0], [] );
	init_dropped_test!( [u8; 1], [POST_DROP_U8] );

	init_dropped_test!( [u32; 0], [] );
	init_dropped_test!( [u32; 1], [POST_DROP_U32] );
    }
}
