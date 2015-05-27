#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::convert::AsRef;

    // impl AsRef<str> for str {
    //     #[inline]
    //     fn as_ref(&self) -> &str {
    //         self
    //     }
    // }

    // impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a T where T: AsRef<U> {
    //     fn as_ref(&self) -> &U {
    //         <T as AsRef<U>>::as_ref(*self)
    //     }
    // }

    type T = i32;

    #[test]
    fn as_ref_test1() {
	let hello: &str = "Hello, World!";
	let as_ref: &str = (*hello).as_ref();

	assert_eq!(as_ref, hello);
    }

    #[test]
    fn as_ref_test2() {
	let hello: &str = "Hello, World!";
	let as_ref: &str = hello.as_ref();

	assert_eq!(as_ref, hello);
    }
}
