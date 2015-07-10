#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::nonzero::NonZero;

    // #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
    // #[unstable(feature = "core")]
    // pub struct NonZero<T: Zeroable>(T);

    // impl<T: Zeroable> NonZero<T> {
    //     /// Creates an instance of NonZero with the provided value.
    //     /// You must indeed ensure that the value is actually "non-zero".
    //     #[inline(always)]
    //     pub unsafe fn new(inner: T) -> NonZero<T> {
    //         NonZero(inner)
    //     }
    // }

    type T = i32; // T: Zeroable

    #[test]
    fn new_test1() {
	let inner: T = 68; // non-zero
	let _: NonZero<T> = unsafe { NonZero::<T>::new(inner) };
    }
}
