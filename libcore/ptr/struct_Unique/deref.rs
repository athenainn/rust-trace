#![feature(core, unique)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::Unique;

    use core::ops::Deref;

    // pub struct Unique<T: ?Sized> {
    //     pointer: NonZero<*const T>,
    //     _marker: PhantomData<T>,
    // }

    // unsafe impl<T: Send + ?Sized> Send for Unique<T> { }

    // unsafe impl<T: Sync + ?Sized> Sync for Unique<T> { }

    // impl<T:?Sized> Deref for Unique<T> {
    //     type Target = *mut T;
    //
    //     #[inline]
    //     fn deref<'a>(&'a self) -> &'a *mut T {
    //         unsafe { mem::transmute(&*self.pointer) }
    //     }
    // }

    type T = i32;

    #[test]
    fn deref_test1() {
	let mut x: T = 68;
	let unique: Unique<T> = unsafe { Unique::<T>::new(&mut x) };
	let result: &*mut T = unique.deref();

	unsafe { **result = 500 };

	assert_eq!(x, 500);
    }

    #[test]
    fn deref_test2() {
	let mut x: T = 68;
	let unique: Unique<T> = unsafe { Unique::<T>::new(&mut x) };
	let result: *mut T = *unique;

	unsafe { *result = 500 };

	assert_eq!(x, 500);
    }
}
