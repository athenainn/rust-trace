#![feature(core, core_intrinsics, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::ToOwned;

    use core::default::Default;

    use core::clone::Clone;

    use core::intrinsics::type_name;

    // pub trait ToOwned {
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Owned: Borrow<Self>;
    //
    //     /// Creates owned data from borrowed data, usually by cloning.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn to_owned(&self) -> Self::Owned;
    // }

    // impl<T> ToOwned for T where T: Clone {
    //     type Owned = T;
    //     fn to_owned(&self) -> T { self.clone() }
    // }

    // pub trait Borrow<Borrowed: ?Sized> {
    //     /// Immutably borrows from an owned value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::Borrow;
    //     ///
    //     /// fn check<T: Borrow<str>>(s: T) {
    //     ///     assert_eq!("Hello", s.borrow());
    //     /// }
    //     ///
    //     /// let s = "Hello".to_string();
    //     ///
    //     /// check(s);
    //     ///
    //     /// let s = "Hello";
    //     ///
    //     /// check(s);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn borrow(&self) -> &Borrowed;
    // }

    struct A<T> {
	value: T
    }

    impl<T> A<T> {
	fn new(value: T) -> Self {
	    A { value: value }
	}
    }

    impl<T: Default> Default for A<T> {
	fn default() -> Self {
	    Self::new(Default::default())
	}
    }

    impl<T: Clone> Clone for A<T> {
	fn clone(&self) -> Self {
	    Self::new(self.value.clone())
	}
    }

    type T = A<i32>; // T: clone

    #[test]
    fn owned_test1() {
	let typename: &'static str = unsafe {
	    type_name::<<T as ToOwned>::Owned>()
	};

	assert_eq!(typename, "tests::A<i32>");
    }
}
