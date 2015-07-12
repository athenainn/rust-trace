#![feature(core, alloc)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use core::cmp::Ordering::{self, Less, Equal, Greater};

    // pub struct Box<T>(Unique<T>);

    // impl<T: ?Sized + PartialOrd> PartialOrd for Box<T> {
    //     #[inline]
    //     fn partial_cmp(&self, other: &Box<T>) -> Option<Ordering> {
    //         PartialOrd::partial_cmp(&**self, &**other)
    //     }
    //     #[inline]
    //     fn lt(&self, other: &Box<T>) -> bool { PartialOrd::lt(&**self, &**other) }
    //     #[inline]
    //     fn le(&self, other: &Box<T>) -> bool { PartialOrd::le(&**self, &**other) }
    //     #[inline]
    //     fn ge(&self, other: &Box<T>) -> bool { PartialOrd::ge(&**self, &**other) }
    //     #[inline]
    //     fn gt(&self, other: &Box<T>) -> bool { PartialOrd::gt(&**self, &**other) }
    // }

    type T = i32; // T: ?Sized + PartialOrd

    macro_rules! partial_cmp_test {
	($T:ty) => ({
	    {
		let b: Box<$T> = Box::<$T>::new(68);
		let other: Box<$T> = Box::<$T>::new(500);
		let result: Option<Ordering> = b.partial_cmp(&other);

		match result {
		    Some(Less) => assert!(true),
		    _ => assert!(false)
		}
	    }
	    {
		let b: Box<$T> = Box::<$T>::new(68);
		let other: Box<$T> = Box::<$T>::new(68);
		let result: Option<Ordering> = b.partial_cmp(&other);

		match result {
		    Some(Equal) => assert!(true),
		    _ => assert!(false)
		}
	    }
	    {
		let b: Box<$T> = Box::<$T>::new(500);
		let other: Box<$T> = Box::<$T>::new(68);
		let result: Option<Ordering> = b.partial_cmp(&other);

		match result {
		    Some(Greater) => assert!(true),
		    _ => assert!(false)
		}
	    }
	})
    }

    #[test]
    fn partial_cmp_test1() {
	partial_cmp_test!( T );
    }
}
