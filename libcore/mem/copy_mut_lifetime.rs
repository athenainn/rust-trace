#![feature(core, copy_lifetime)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::copy_mut_lifetime;

    use core::marker::PhantomData;

    // pub unsafe fn copy_mut_lifetime<'a, S: ?Sized, T: ?Sized + 'a>(_ptr: &'a S,
    //                                                                ptr: &mut T)
    //                                                               -> &'a mut T
    // {
    //     transmute(ptr)
    // }

    struct Bar<T> {
	bar: T
    }

    struct Foo<'a, T: 'a> {
	ptr: *mut Bar<T>,
	marker: PhantomData<&'a T>
    }

    impl<'a, T> Foo<'a, T> {
	fn bar(&'a self) -> &'a mut T {
	    let ptr: &mut T = unsafe { &mut (*self.ptr).bar };
	    unsafe { copy_mut_lifetime(self, &mut *ptr) }
	}
    }

    type T = i32;

    #[test]
    fn copy_lifetime_test1() {
	let mut bar: Bar<T> = Bar { bar: 68 };
	let foo: Foo<T> = Foo { ptr: &mut bar, marker: PhantomData::<&T> };
	let ptr: &mut T = foo.bar();

	assert_eq!(*ptr, 68);
    }
}
