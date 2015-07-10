#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::forget;
    use core::mem::uninitialized;

    use core::ops::Drop;

    // pub fn forget<T>(t: T) {
    //     unsafe { intrinsics::forget(t) }
    // }

    static mut dtor_count: usize = 0;

    struct A;

    impl Drop for A {
	fn drop(&mut self) {
	    unsafe { dtor_count += 1 };
	}
    }

    type T = i32;

    fn foo(_: &A) {
	let t: A = unsafe { uninitialized::<A>() };
	forget::<A>(t);
    }

    #[test]
    fn forget_test1() {
	{
	    let x: A = A;
	    foo(&x);
	}

	assert_eq!(unsafe { dtor_count }, 1);
    }
}
