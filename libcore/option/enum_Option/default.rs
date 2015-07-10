#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::default::Default;

    // #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Option<T> {
    //     /// No value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     None,
    //     /// Some value `T`
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Some(T)
    // }

    // impl<T> Default for Option<T> {
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn default() -> Option<T> { None }
    // }

    type T = u32;

    #[test]
    fn default_test1() {
	let x: Option<T> = Option::<T>::default();

	assert_eq!(x, None::<T>);
    }

    #[test]
    fn default_test2() {
	let x: Option<T> = Default::default();

	assert_eq!(x, None::<T>);
    }
}
