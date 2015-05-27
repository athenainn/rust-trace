#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::convert::AsMut;

    // pub trait AsMut<T: ?Sized> {
    //     /// Performs the conversion.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn as_mut(&mut self) -> &mut T;
    // }

    // impl<'a, T: ?Sized, U: ?Sized> AsMut<U> for &'a mut T where T: AsMut<U> {
    //     fn as_mut(&mut self) -> &mut U {
    //         (*self).as_mut()
    //     }
    // }

    type T = i32;

    #[test]
    fn as_mut_test1() {
	let mut slice: &mut [T] = &mut [68, 500, 999];
	let _: &mut [T] = (*slice).as_mut();
    }

    #[test]
    fn as_mut_test2() {
	let mut slice: &mut [T] = &mut [68, 500, 999];
	let _: &mut [T] = slice.as_mut();
    }
}
