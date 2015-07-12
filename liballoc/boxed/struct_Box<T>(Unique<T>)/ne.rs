#![feature(alloc)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    // pub struct Box<T>(Unique<T>);

    // impl<T: ?Sized + PartialEq> PartialEq for Box<T> {
    //     #[inline]
    //     fn eq(&self, other: &Box<T>) -> bool { PartialEq::eq(&**self, &**other) }
    //     #[inline]
    //     fn ne(&self, other: &Box<T>) -> bool { PartialEq::ne(&**self, &**other) }
    // }

    type T = i32; // T: ?Sized + PartialEq

    macro_rules! ne_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let b: Box<$T> = Box::<$T>::new($value);
	    let other: Box<$T> = Box::<$T>::new($other);

	    assert_eq!(b.ne(&other), $result);
	    assert_eq!(b != other, $result);
	})
    }

    #[test]
    fn ne_test1() {
	ne_test!( T, 68, 68, false );
	ne_test!( T, 68, 500, true );
	ne_test!( T, 500, 68, true );
	ne_test!( T, 500, 500, false );
    }
}
