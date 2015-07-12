#![feature(core, alloc)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use core::any::Any;

    use std::thread;

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

    // impl Box<Any + Send> {
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     /// Attempt to downcast the box to a concrete type.
    //     pub fn downcast<T: Any>(self) -> Result<Box<T>, Box<Any + Send>> {
    //         <Box<Any>>::downcast(self).map_err(|s| unsafe {
    //             // reapply the Send marker
    //             mem::transmute::<Box<Any>, Box<Any + Send>>(s)
    //         })
    //     }
    // }

    #[derive(Debug, PartialEq)]
    struct A<U> {
	value: U
    }

    impl<U> A<U> {
	fn new(x: U) -> Self {
	    A { value: x }
	}
    }

    type U = i32;
    type T = A<U>; // T: Any + Send

    #[test]
    fn downcast_test1() {
	let x: T = <T>::new(68);
	let b: Box<Any + Send> = Box::<T>::new(x);

	thread::spawn(|| {
	    let result: Result<Box<T>, Box<Any + Send>> = b.downcast::<T>();

	    match result {
		Ok(v) => assert_eq!(v.value, 68),
		Err(_) => assert!(false)
	    }
	});

	thread::sleep_ms(10);
    }

    #[test]
    fn downcast_test2() {
	let x: T = <T>::new(68);
	let b: Box<Any + Send> = Box::<T>::new(x);

	thread::spawn(|| {
	    type V = &'static T;
	    let result: Result<Box<V>, Box<Any + Send>> = b.downcast::<V>();

	    match result {
		Ok(_) => assert!(false),
		Err(_) => assert!(true)
	    }
	});

	thread::sleep_ms(10);
    }
}
