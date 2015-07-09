#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::move_val_init;

    use core::default::Default;

    // pub fn move_val_init<T>(dst: &mut T, src: T);

    macro_rules! move_val_init_test {
	($T:ty, $value:expr) => ({
	    let mut value: $T = Default::default();

	    {
		let dst: &mut $T = &mut value;
		let src: $T = $value;

		unsafe { move_val_init::<$T>(dst, src); }
	    }

	    assert_eq!(value, $value);
	})
    }

    #[test]
    fn move_val_init_test1() {
	move_val_init_test!( usize, 68 );
    }
}
