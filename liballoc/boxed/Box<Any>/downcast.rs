#![feature(core, alloc)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use core::any::Any;

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

    // impl Box<Any> {
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     /// Attempt to downcast the box to a concrete type.
    //     pub fn downcast<T: Any>(self) -> Result<Box<T>, Box<Any>> {
    //         if self.is::<T>() {
    //             unsafe {
    //                 // Get the raw representation of the trait object
    //                 let raw = Box::into_raw(self);
    //                 let to: TraitObject =
    //                     mem::transmute::<*mut Any, TraitObject>(raw);
    //
    //                 // Extract the data pointer
    //                 Ok(Box::from_raw(to.data as *mut T))
    //             }
    //         } else {
    //             Err(self)
    //         }
    //     }
    // }

    #[derive(Debug)]
    struct A<U> {
	value: U
    }

    impl<U> A<U> {
	fn new(x: U) -> Self {
	    A { value: x }
	}
    }

    type U = i32;
    type T = A<U>; // T: Any

    #[test]
    fn downcast_test1() {
	let x: T = <T>::new(68);
	let b: Box<Any> = Box::<T>::new(x);

	let result: Result<Box<T>, Box<Any>> = b.downcast::<T>();

	match result {
	    Ok(v) => assert_eq!(v.value, 68),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn downcast_test2() {
	let x: T = <T>::new(68);
	let b: Box<Any> = Box::<T>::new(x);

	type U = &'static T;
	let result: Result<Box<U>, Box<Any>> = b.downcast::<U>();

	match result {
	    Ok(_) => assert!(false),
	    Err(_) => assert!(true)
	}
    }
}
