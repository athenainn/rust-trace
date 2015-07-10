#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::align_of_val;

    use core::default::Default;

    // pub fn align_of_val<T: ?Sized>(val: &T) -> usize {
    //     unsafe { intrinsics::min_align_of_val(val) }
    // }

    macro_rules! align_of_val_test {
	($T:ty, $size:expr) => ({
	    let v: $T = Default::default();
	    let size: usize = align_of_val::<$T>(&v);

	    assert_eq!(size, $size);
	})
    }

    #[test]
    fn align_of_val_test1() {
	struct A;

	let a: A = A;
	let size: usize = align_of_val::<A>(&a);

	assert_eq!(size, 1);
    }

    #[test]
    fn align_of_val_test2() {
	align_of_val_test!( (u8), 1 );
	align_of_val_test!( (u8, u16), 2 );
	align_of_val_test!( (u8, u16, u32), 4 );
	align_of_val_test!( (u8, u16, u32, u64), 8 );
    }
}
