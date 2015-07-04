#![feature(core, unique)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::Unique;

    // pub struct Unique<T: ?Sized> {
    //     pointer: NonZero<*const T>,
    //     _marker: PhantomData<T>,
    // }

    // unsafe impl<T: Send + ?Sized> Send for Unique<T> { }

    // unsafe impl<T: Sync + ?Sized> Sync for Unique<T> { }

    // impl<T: ?Sized> Unique<T> {
    //     /// Creates a new `Unique`.
    //     #[unstable(feature = "unique")]
    //     pub unsafe fn new(ptr: *mut T) -> Unique<T> {
    //         Unique { pointer: NonZero::new(ptr), _marker: PhantomData }
    //     }
    //
    //     /// Dereferences the content.
    //     #[unstable(feature = "unique")]
    //     pub unsafe fn get(&self) -> &T {
    //         &**self.pointer
    //     }
    //
    //     /// Mutably dereferences the content.
    //     #[unstable(feature = "unique")]
    //     pub unsafe fn get_mut(&mut self) -> &mut T {
    //         &mut ***self
    //     }
    // }

    type T = i32;

    #[test]
    fn get_mut_test1() {
	let mut x: T = 68;
	let mut unique: Unique<T> = unsafe { Unique::<T>::new(&mut x) };
	let ptr: &mut T = unsafe { unique.get_mut() };

	assert_eq!(ptr.eq(&&x), true);

	*ptr = 500;

	assert_eq!(x, 500);
    }
}
