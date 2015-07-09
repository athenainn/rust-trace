#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::min_align_of_val;

    use core::default::Default;

    // pub fn min_align_of_val<T: ?Sized>(_: &T) -> usize;

    macro_rules! min_align_of_val_test {
	($T:ty, $size:expr) => ({
	    let v: $T = Default::default();
	    let size: usize = unsafe { min_align_of_val::<$T>(&v) };

	    assert_eq!(size, $size);
	})
    }

    #[test]
    fn min_align_of_val_test1() {
	min_align_of_val_test!( (u8), 1 );
	min_align_of_val_test!( (u8, u16), 2 );
	min_align_of_val_test!( (u8, u16, u32), 4 );
	min_align_of_val_test!( (u8, u16, u32, u64), 8 );
    }
}
