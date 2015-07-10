#![feature(core, copy_lifetime)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::copy_lifetime;

    use core::marker::PhantomData;

    // pub unsafe fn copy_lifetime<'a, S: ?Sized, T: ?Sized + 'a>(_ptr: &'a S,
    //                                                         ptr: &T) -> &'a T {
    //     transmute(ptr)
    // }

    struct Bar<T> {
	bar: T
    }

    struct Foo<'a, T: 'a> {
	ptr: *const Bar<T>,
	marker: PhantomData<&'a T>
    }

    impl<'a, T> Foo<'a, T> {
	fn bar(&'a self) -> &'a T {
	    let ptr: &T = unsafe { &(*self.ptr).bar };
	    unsafe { copy_lifetime(self, &*ptr) }
	}
    }

    type T = i32;

    #[test]
    fn copy_lifetime_test1() {
	let bar: Bar<T> = Bar { bar: 68 };
	let foo: Foo<T> = Foo { ptr: &bar, marker: PhantomData::<&T> };
	let ptr: &T = foo.bar();

	assert_eq!(*ptr, 68);
    }
}
