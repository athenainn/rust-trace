#![feature(alloc)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    // pub struct Box<T>(Unique<T>);

    // impl<T> Box<T> {
    //     /// Allocates memory on the heap and then moves `x` into it.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Box::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline(always)]
    //     pub fn new(x: T) -> Box<T> {
    //         box x
    //     }
    // }

    // impl<T: fmt::Display + ?Sized> fmt::Display for Box<T> {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         fmt::Display::fmt(&**self, f)
    //     }
    // }

    // impl<T: fmt::Debug + ?Sized> fmt::Debug for Box<T> {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         fmt::Debug::fmt(&**self, f)
    //     }
    // }

    // impl<T> fmt::Pointer for Box<T> {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         // It's not possible to extract the inner Uniq directly from the Box,
    //         // instead we cast it to a *const which aliases the Unique
    //         let ptr: *const T = &**self;
    //         fmt::Pointer::fmt(&ptr, f)
    //     }
    // }

    type T = i32; // T: fmt::Display + ? Sized
		  // T: fmt::Debug + ?Sized

    #[test]
    fn fmt_test1() {
	let x: T = 68;
	let b: Box<T> = Box::<T>::new(x);
	let message: String = format!("{}", b);

	assert_eq!(message, "68".to_string());
    }

    #[test]
    fn fmt_test2() {
	let x: T = 68;
	let b: Box<T> = Box::<T>::new(x);
	let message: String = format!("{:?}", b);

	assert_eq!(message, "68".to_string());
    }

    #[test]
    fn fmt_test3() {
	let x: T = 68;
	let b: Box<T> = Box::<T>::new(x);
	let _: String = format!("{:p}", b); // `"0x104c25010"`
    }
}
