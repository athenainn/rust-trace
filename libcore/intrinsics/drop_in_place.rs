#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::drop_in_place;

    use core::default::Default;

    // pub fn drop_in_place<T: ?Sized>(_: *mut T);

    macro_rules! drop_in_place_test {
	($T:ty) => ({
	    let mut value: $T = Default::default();

	    {
		let mut v: $T = Default::default();
		let p: *mut $T = &mut v;

		unsafe { drop_in_place::<$T>(p); }
	    }
	})
    }

    #[test]
    fn drop_in_place_test1() {
	drop_in_place_test!( u8 );
	drop_in_place_test!( u16 );
	drop_in_place_test!( u32 );
	drop_in_place_test!( u64 );
	drop_in_place_test!( i8 );
	drop_in_place_test!( i16 );
	drop_in_place_test!( i32 );
	drop_in_place_test!( i64 );

	drop_in_place_test!( f32 );
	drop_in_place_test!( f64 );
    }
}
