#![feature(core, alloc)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use core::cmp::Ordering::{self, Less, Equal, Greater};

    // pub struct Box<T>(Unique<T>);

    // impl<T: ?Sized + Ord> Ord for Box<T> {
    //     #[inline]
    //     fn cmp(&self, other: &Box<T>) -> Ordering {
    //         Ord::cmp(&**self, &**other)
    //     }
    // }

    type T = i32; // T: ?Sized + Ord

    macro_rules! cmp_test {
	($T:ty) => ({
	    {
		let b: Box<$T> = Box::<$T>::new(68);
		let other: Box<$T> = Box::<$T>::new(500);
		let result: Ordering = b.cmp(&other);

		assert_eq!(result, Less);
	    }
	    {
		let b: Box<$T> = Box::<$T>::new(68);
		let other: Box<$T> = Box::<$T>::new(68);
		let result: Ordering = b.cmp(&other);

		assert_eq!(result, Equal);
	    }
	    {
		let b: Box<$T> = Box::<$T>::new(500);
		let other: Box<$T> = Box::<$T>::new(68);
		let result: Ordering = b.cmp(&other);

		assert_eq!(result, Greater);
	    }
	})
    }

    #[test]
    fn cmp_test1() {
	cmp_test!( T );
    }
}
