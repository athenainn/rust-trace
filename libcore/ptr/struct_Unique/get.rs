#![feature(core, unique)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::Unique;

    // pub struct Unique<T: ?Sized> {
    //     pointer: NonZero<*const T>,
    //     _marker: PhantomData<T>,
    // }

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
    fn get_test1() {
	let mut x: T = 68;
	let unique: Unique<T> = unsafe { Unique::<T>::new(&mut x) };
	let ptr1: &T = unsafe { unique.get() };
	let ptr2: &T = unsafe { unique.get() };

	assert_eq!(ptr1.eq(&x), true);
	assert_eq!(ptr1.eq(ptr2), true);
    }
}
