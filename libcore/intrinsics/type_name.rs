#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::type_name;

    // pub fn type_name<T>() -> usize;

    macro_rules! type_name_test {
	($T:ty, $message:expr) => ({
	    let message: &'static str = unsafe { type_name::<$T>() };

	    assert_eq!(message, $message);
	})
    }

    #[test]
    fn type_name_test1() {
	type_name_test!( u8, "u8" );
	type_name_test!( u16, "u16" );
	type_name_test!( u32, "u32" );
	type_name_test!( u64, "u64" );
	type_name_test!( i8, "i8" );
	type_name_test!( i16, "i16" );
	type_name_test!( i32, "i32" );
	type_name_test!( i64, "i64" );

	type_name_test!( f32, "f32" );
	type_name_test!( f64, "f64" );

	type_name_test!( [u8; 0], "[u8; 0]" );
	type_name_test!( [u8; 68], "[u8; 68]" );

	type_name_test!( [u32; 0], "[u32; 0]" );
	type_name_test!( [u32; 68], "[u32; 68]" );

	type_name_test!( (u8,), "(u8,)" );
	type_name_test!( (u8, u16), "(u8, u16)" );
	type_name_test!( (u8, u16, u32), "(u8, u16, u32)" );
	type_name_test!( (u8, u16, u32, u64), "(u8, u16, u32, u64)" );
    }
}
