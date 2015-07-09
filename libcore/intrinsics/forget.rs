#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::forget;
    use core::intrinsics::init;

    // pub fn forget<T>(_: T) -> ();

    macro_rules! forget_test {
	($T:ty) => ({
	    let value: $T = unsafe { init::<$T>() };
	    unsafe { forget::<$T>(value); }
	})
    }

    #[test]
    fn forget_test1() {
	forget_test!( u8 );
	forget_test!( u16 );
	forget_test!( u32 );
	forget_test!( u64 );
	forget_test!( i8 );
	forget_test!( i16 );
	forget_test!( i32 );
	forget_test!( i64 );

	forget_test!( f32 );
	forget_test!( f64 );

	forget_test!( [u8; 0] );
	forget_test!( [u8; 1] );

	forget_test!( [u32; 0] );
	forget_test!( [u32; 1] );
    }
}
