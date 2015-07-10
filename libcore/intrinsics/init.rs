#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::init;

    // pub fn init<T>() -> T;

    macro_rules! init_test {
	($T:ty, $value:expr) => ({
	    let value: $T = unsafe { init::<$T>() };

	    assert_eq!(value, $value);
	})
    }

    #[test]
    fn init_test1() {
	init_test!( u8, 0 );
	init_test!( u16, 0 );
	init_test!( u32, 0 );
	init_test!( u64, 0 );
	init_test!( i8, 0 );
	init_test!( i16, 0 );
	init_test!( i32, 0 );
	init_test!( i64, 0 );

	init_test!( f32, 0.0 );
	init_test!( f64, 0.0 );

	init_test!( [u8; 0], [] );
	init_test!( [u8; 1], [0] );

	init_test!( [u32; 0], [] );
	init_test!( [u32; 1], [0] );
    }
}
