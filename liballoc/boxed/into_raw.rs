#![feature(alloc, box_raw)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::into_raw;
    use alloc::boxed::Box;

    // pub struct Box<T>(Unique<T>);

    // impl<T : ?Sized> Box<T> {
    //     /// Constructs a box from the raw pointer.
    //     ///
    //     /// After this function call, pointer is owned by resulting box.
    //     /// In particular, it means that `Box` destructor calls destructor
    //     /// of `T` and releases memory. Since the way `Box` allocates and
    //     /// releases memory is unspecified, the only valid pointer to pass
    //     /// to this function is the one taken from another `Box` with
    //     /// `Box::into_raw` function.
    //     ///
    //     /// Function is unsafe, because improper use of this function may
    //     /// lead to memory problems like double-free, for example if the
    //     /// function is called twice on the same raw pointer.
    //     #[unstable(feature = "box_raw",
    //                reason = "may be renamed or moved out of Box scope")]
    //     #[inline]
    //     // NB: may want to be called from_ptr, see comments on CStr::from_ptr
    //     pub unsafe fn from_raw(raw: *mut T) -> Self {
    //         mem::transmute(raw)
    //     }
    //
    //     /// Consumes the `Box`, returning the wrapped raw pointer.
    //     ///
    //     /// After call to this function, caller is responsible for the memory
    //     /// previously managed by `Box`, in particular caller should properly
    //     /// destroy `T` and release memory. The proper way to do it is to
    //     /// convert pointer back to `Box` with `Box::from_raw` function, because
    //     /// `Box` does not specify, how memory is allocated.
    //     ///
    //     /// # Examples
    //     /// ```
    //     /// # #![feature(box_raw)]
    //     /// let seventeen = Box::new(17u32);
    //     /// let raw = Box::into_raw(seventeen);
    //     /// let boxed_again = unsafe { Box::from_raw(raw) };
    //     /// ```
    //     #[unstable(feature = "box_raw", reason = "may be renamed")]
    //     #[inline]
    //     // NB: may want to be called into_ptr, see comments on CStr::from_ptr
    //     pub fn into_raw(b: Box<T>) -> *mut T {
    //         unsafe { mem::transmute(b) }
    //     }
    // }

    // pub fn into_raw<T : ?Sized>(b: Box<T>) -> *mut T {
    //     Box::into_raw(b)
    // }

    type T = i32; // T: ?Sized

    #[test]
    fn into_raw_test1() {
	let x: T = 68;
	let b: Box<T> = Box::<T>::new(x);

	let raw: *mut T = into_raw(b);
	unsafe { *raw = 500; }

	let b2: Box<T> = unsafe { Box::<T>::from_raw(raw) };
	assert_eq!(*b2, 500);
    }
}
