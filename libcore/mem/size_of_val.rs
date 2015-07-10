#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::size_of_val;

    // pub fn size_of_val<T: ?Sized>(val: &T) -> usize {
    //     unsafe { intrinsics::size_of_val(val) }
    // }

    macro_rules! size_of_val_test {
	($T:ty, $init:expr, $size:expr) => ({
	    let v: $T = $init;
	    let size: usize = size_of_val::<$T>(&v);

	    assert_eq!(size, $size);
	})
    }

    #[test]
    fn size_of_val_test1() {
	struct A;

	let a: A = A;

	size_of_val_test!( A, a, 0 );
    }

    #[test]
    fn size_of_val_test2() {
	size_of_val_test!( u8, 0, 1 );
	size_of_val_test!( u16, 0, 2 );
	size_of_val_test!( u32, 0, 4 );
	size_of_val_test!( u64, 0, 8 );

	size_of_val_test!( i8, 0, 1 );
	size_of_val_test!( i16, 0, 2 );
	size_of_val_test!( i32, 0, 4 );
	size_of_val_test!( i64, 0, 8 );

	size_of_val_test!( f32, 0.0, 4 );
	size_of_val_test!( f64, 0.0, 8 );
    }
}
