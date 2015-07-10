#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::forget;
    use core::intrinsics::init_dropped;
    use core::intrinsics::init;
    use core::intrinsics::uninit;

    use core::ops::Drop;

    // pub fn forget<T>(_: T) -> ();

    macro_rules! forget_test {
	($T:ty) => ({
	    let value: $T = unsafe { init_dropped::<$T>() };
	    unsafe { forget::<$T>(value); }

	    let value: $T = unsafe { init::<$T>() };
	    unsafe { forget::<$T>(value); }

	    let value: $T = unsafe { uninit::<$T>() };
	    unsafe { forget::<$T>(value); }
	})
    }

    #[test]
    fn forget_test1() {
	struct A;

	impl Drop for A {
	    fn drop(&mut self) {
		assert!(false)
	    }
	}

	forget_test!( A );
    }
}
