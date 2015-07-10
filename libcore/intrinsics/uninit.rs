#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::uninit;

    // pub fn uninit<T>() -> T;

    macro_rules! uninit_test {
	($T:ty) => ({
	    let _: $T = unsafe { uninit::<$T>() };
	})
    }

    #[test]
    fn uninit_test1() {
	uninit_test!( u8 );
	uninit_test!( u16 );
	uninit_test!( u32 );
	uninit_test!( u64 );
	uninit_test!( i8 );
	uninit_test!( i16 );
	uninit_test!( i32 );
	uninit_test!( i64 );

	uninit_test!( f32 );
	uninit_test!( f64 );
    }
}
