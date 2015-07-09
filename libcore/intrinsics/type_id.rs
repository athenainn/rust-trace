#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::type_id;

    // pub fn type_id<T>() -> usize;

    macro_rules! type_id_test {
	($T:ty, $id:expr) => ({
	    let id: u64 = unsafe { type_id::<$T>() };

	    assert_eq!(id, $id);
	})
    }

    #[test]
    fn type_id_test1() {
	type_id_test!( u8, 15275277969672056570 );
	type_id_test!( u16, 18053301371599279620 );
	type_id_test!( u32, 18227924320386998543 );
	type_id_test!( u64, 1452060325351959634 );
	type_id_test!( i8, 6422278243338867694 );
	type_id_test!( i16, 12335411354326379021 );
	type_id_test!( i32, 10645063183773766558 );
	type_id_test!( i64, 13485345776957239514 );

	type_id_test!( f32, 9802228723463931751 );
	type_id_test!( f64, 17162490774200000607 );

	type_id_test!( [u8; 0], 3036919213479723430 );
	type_id_test!( [u8; 68], 14606272329460345633 );

	type_id_test!( [u32; 0], 8739390100933943764 );
	type_id_test!( [u32; 68], 11567740553446811297 );

	type_id_test!( (u8,), 13285508652181658211 );
	type_id_test!( (u8, u16), 7669834128470506113 );
	type_id_test!( (u8, u16, u32), 7307969372154648016 );
	type_id_test!( (u8, u16, u32, u64), 11973463677228854774 );
    }
}
