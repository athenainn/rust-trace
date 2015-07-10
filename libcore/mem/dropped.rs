#![feature(core, filling_drop)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::dropped;

    use core::mem::POST_DROP_U8;
    use core::mem::POST_DROP_U32;
    use core::mem::POST_DROP_U64;
    use core::mem::POST_DROP_USIZE;

    // pub unsafe fn dropped<T>() -> T {
    //     #[inline(always)]
    //     unsafe fn dropped_impl<T>() -> T { intrinsics::init_dropped() }
    //
    //     dropped_impl()
    // }

    // pub const POST_DROP_U8: u8 = 0x1d;
    // pub const POST_DROP_U32: u32 = repeat_u8_as_u32!(POST_DROP_U8);
    // pub const POST_DROP_U64: u64 = repeat_u8_as_u64!(POST_DROP_U8);

    // macro_rules! repeat_u8_as_u32 {
    //     ($name:expr) => { (($name as u32) << 24 |
    //                        ($name as u32) << 16 |
    //                        ($name as u32) <<  8 |
    //                        ($name as u32)) }
    // }
    // macro_rules! repeat_u8_as_u64 {
    //     ($name:expr) => { ((repeat_u8_as_u32!($name) as u64) << 32 |
    //                        (repeat_u8_as_u32!($name) as u64)) }
    // }

    // #[cfg(target_pointer_width = "32")]
    // #[unstable(feature = "filling_drop")]
    // #[allow(missing_docs)]
    // pub const POST_DROP_USIZE: usize = POST_DROP_U32 as usize;
    // #[cfg(target_pointer_width = "64")]
    // #[unstable(feature = "filling_drop")]
    // #[allow(missing_docs)]
    // pub const POST_DROP_USIZE: usize = POST_DROP_U64 as usize;

    macro_rules! dropped_test {
	($T:ty, $value:expr) => ({
	    let value: $T = unsafe { dropped::<$T>() };

	    assert_eq!(value, $value);
	})
    }

    #[test]
    fn dropped_test1() {
	dropped_test!( u8, POST_DROP_U8 );
	dropped_test!( u16, 0x1d1d );
	dropped_test!( u32, POST_DROP_U32 );
	dropped_test!( u64, POST_DROP_U64 );
	dropped_test!( usize, POST_DROP_USIZE );

	dropped_test!( i8, POST_DROP_U8 as i8 );
	dropped_test!( i16, 0x1d1d );
	dropped_test!( i32, POST_DROP_U32 as i32 );
	dropped_test!( i64, POST_DROP_U64 as i64 );
	dropped_test!( isize, POST_DROP_USIZE as isize );

	dropped_test!( f32, unsafe {
	    *(&0x1d1d1d1d_u32 as *const u32 as *const f32)
	} );
	dropped_test!( f64, unsafe {
	    *(&0x1d1d1d1d1d1d1d1d_u64 as *const u64 as *const f64)
	} );

	dropped_test!( [u8; 0], [] );
	dropped_test!( [u8; 1], [POST_DROP_U8] );

	dropped_test!( [u32; 0], [] );
	dropped_test!( [u32; 1], [POST_DROP_U32] );
    }
}
