#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::uninitialized;

    // pub unsafe fn uninitialized<T>() -> T {
    //     intrinsics::uninit()
    // }

    macro_rules! uninitialized_test {
	($T:ty) => ({
	    let _: $T = unsafe { uninitialized::<$T>() };
	})
    }

    #[test]
    fn uninitialized_test1() {
	uninitialized_test!( u8 );
	uninitialized_test!( u16 );
	uninitialized_test!( u32 );
	uninitialized_test!( u64 );
	uninitialized_test!( usize );

	uninitialized_test!( i8 );
	uninitialized_test!( i16 );
	uninitialized_test!( i32 );
	uninitialized_test!( i64 );
	uninitialized_test!( isize );

	uninitialized_test!( f32 );
	uninitialized_test!( f64 );

	uninitialized_test!( [u8; 0] );
	uninitialized_test!( [u8; 1] );

	uninitialized_test!( [u32; 0] );
	uninitialized_test!( [u32; 1] );
    }
}
