#![feature(alloc)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    // pub struct Box<T>(Unique<T>);

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T: Default> Default for Box<T> {
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn default() -> Box<T> { box Default::default() }
    // }

    type T = i32;

    #[test]
    fn default_test1() {
	let b: Box<T> = Box::<T>::default();

	assert_eq!(*b, 0);
    }
}
