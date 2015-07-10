#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::nonzero::NonZero;

    use core::ops::Deref;

    // #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
    // #[unstable(feature = "core")]
    // pub struct NonZero<T: Zeroable>(T);

    // impl<T: Zeroable> Deref for NonZero<T> {
    //     type Target = T;
    //
    //     #[inline]
    //     fn deref<'a>(&'a self) -> &'a T {
    //         let NonZero(ref inner) = *self;
    //         inner
    //     }
    // }

    type T = i32; // T: Zeroable

    #[test]
    fn deref_test1() {
	let inner: T = 68; // non-zero
	let nonzero: NonZero<T> = unsafe { NonZero::<T>::new(inner) };
	let deref: &T = nonzero.deref();

	assert_eq!(deref, &68);
    }

    #[test]
    fn deref_test2() {
	let inner: T = 68; // non-zero
	let nonzero: NonZero<T> = unsafe { NonZero::<T>::new(inner) };
	let deref: T = *nonzero;

	assert_eq!(deref, 68);
    }
}
