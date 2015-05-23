#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::RangeFrom;
    use core::ops::Index;

    use core::raw::Slice;
    use core::raw::Repr;

    use core::intrinsics::transmute;

    struct A<T> {
	storage: Slice<T>
    }

    impl A<T> {
	fn len(&self) -> usize {
	    self.storage.len
	}

	fn as_ptr(&self) -> *const T {
	    self.storage.data
	}
    }

    // pub struct RangeFrom<Idx> {
    //     /// The lower bound of the range (inclusive).
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub start: Idx,
    // }

    // pub trait Index<Idx: ?Sized> {
    //     /// The returned type after indexing
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output: ?Sized;
    //
    //     /// The method for the indexing (`Foo[Bar]`) operation
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn index<'a>(&'a self, index: Idx) -> &'a Self::Output;
    // }

    impl Index<RangeFrom<usize>> for A<T> {
	type Output = [T];

	fn index<'a>(&'a self, index: RangeFrom<usize>) -> &'a Self::Output {
	    assert!(index.start <= self.len());

	    unsafe {
		transmute::<Slice<T>, &'a [T]>(
		    Slice {
			data: self.as_ptr().offset(index.start as isize),
			len: self.len() - index.start
		    }
		)
	    }
	}
    }

    type T = i32;

    #[test]
    fn rangefrom_test1() {
	let rangefrom: RangeFrom<T> = RangeFrom { start: 100 };
	let message: String = format!("{:?}", rangefrom);

	assert_eq!(message, "100..".to_string());
    }

    #[test]
    fn rangefrom_test2() {
	let rangefrom: RangeFrom<T> = 100..;
	let message: String = format!("{:?}", rangefrom);

	assert_eq!(message, "100..".to_string());
    }

    #[test]
    fn rangefrom_test3() {
	let a: A<T> = A { storage: [1, 2, 3, 4, 5, 6].repr() };
	let range: RangeFrom<usize> = 2..;
	let slice: &[T] = &a[range];
	let repr: Slice<T> = slice.repr();

	assert_eq!(repr.len, 4);

	for i in 0..repr.len {
	    assert_eq!(slice[i], (i + 3) as T);
	}
    }
}
