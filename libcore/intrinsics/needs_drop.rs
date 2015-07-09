#![feature(core, collections, core_intrinsics)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::intrinsics::needs_drop;

    use collections::vec::Vec;

    // pub fn needs_drop<T>() -> bool;

    macro_rules! needs_drop_test {
	($T:ty, $result:expr) => ({
	    let result: bool = unsafe { needs_drop::<$T>() };

	    assert_eq!(result, $result);
	})
    }

    #[test]
    fn needs_drop_test1() {
	needs_drop_test!( u8, false );
	needs_drop_test!( u16, false );
	needs_drop_test!( u32, false );
	needs_drop_test!( u64, false );
	needs_drop_test!( i8, false );
	needs_drop_test!( i16, false );
	needs_drop_test!( i32, false );
	needs_drop_test!( i64, false );

	needs_drop_test!( f32, false );
	needs_drop_test!( f64, false );

	needs_drop_test!( [u8; 0], false );
	needs_drop_test!( [u8; 1], false );

	needs_drop_test!( [u32; 0], false );
	needs_drop_test!( [u32; 1], false );
    }

    #[test]
    fn needs_drop_test2() {
	needs_drop_test!( Vec<u8>, true );
	needs_drop_test!( Vec<u16>, true );
	needs_drop_test!( Vec<u32>, true );
	needs_drop_test!( Vec<u64>, true );
	needs_drop_test!( Vec<i8>, true );
	needs_drop_test!( Vec<i16>, true );
	needs_drop_test!( Vec<i32>, true );
	needs_drop_test!( Vec<i64>, true );

	needs_drop_test!( Vec<f32>, true );
	needs_drop_test!( Vec<f64>, true );
    }

    #[test]
    fn needs_drop_test3() {
	needs_drop_test!( String, true );
    }
}
