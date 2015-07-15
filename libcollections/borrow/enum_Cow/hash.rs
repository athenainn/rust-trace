#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::Cow::{self, Borrowed, Owned};
    use collections::borrow::Borrow;

    use core::hash::Hash;
    use core::hash::Hasher;
    use core::hash::SipHasher;

    // pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
    //     /// Borrowed data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Borrowed(&'a B),
    //
    //     /// Owned data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Owned(<B as ToOwned>::Owned)
    // }

    // impl<'a, B: ?Sized> Hash for Cow<'a, B> where B: Hash + ToOwned
    // {
    //     #[inline]
    //     fn hash<H: Hasher>(&self, state: &mut H) {
    //         Hash::hash(&**self, state)
    //     }
    // }

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

    // impl<'a, B: ?Sized> Cow<'a, B> where B: ToOwned {
    //     /// Acquires a mutable reference to the owned form of the data.
    //     ///
    //     /// Copies the data if it is not already owned.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::Cow;
    //     ///
    //     /// let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    //     ///
    //     /// let hello = cow.to_mut();
    //     ///
    //     /// assert_eq!(hello, &[1, 2, 3]);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
    //         match *self {
    //             Borrowed(borrowed) => {
    //                 *self = Owned(borrowed.to_owned());
    //                 self.to_mut()
    //             }
    //             Owned(ref mut owned) => owned
    //         }
    //     }
    //
    //     /// Extracts the owned data.
    //     ///
    //     /// Copies the data if it is not already owned.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::Cow;
    //     ///
    //     /// let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    //     ///
    //     /// let hello = cow.into_owned();
    //     ///
    //     /// assert_eq!(vec![1, 2, 3], hello);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn into_owned(self) -> <B as ToOwned>::Owned {
    //         match self {
    //             Borrowed(borrowed) => borrowed.to_owned(),
    //             Owned(owned) => owned
    //         }
    //     }
    // }

    type T = i32; // T: ?Sized + Hash
    type B = T; // B: ToOwned + ?Sized + 'a
    type H = SipHasher; // H: Hasher + Default

    #[test]
    fn hash_test1() {
	let x: T = T::default();
	let x_ref: &T = x.borrow();
	let cow: Cow<B> = Borrowed::<B>(x_ref);
	let mut state: SipHasher = SipHasher::new();

	let finish: u64 = state.finish();
	assert_eq!(finish, 0x1e924b9d737700d7);

	cow.hash::<H>(&mut state);

	let finish: u64 = state.finish();
	assert_eq!(finish, 0x7bf55e51b22b9698);
    }

    #[test]
    fn hash_test2() {
	let x: T = T::default();
	let cow: Cow<B> = Owned::<B>(x);
	let mut state: SipHasher = SipHasher::new();

	let finish: u64 = state.finish();
	assert_eq!(finish, 0x1e924b9d737700d7);

	cow.hash::<H>(&mut state);

	let finish: u64 = state.finish();
	assert_eq!(finish, 0x7bf55e51b22b9698);
    }
}
