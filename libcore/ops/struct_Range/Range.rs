#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Range;
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

    // pub struct Range<Idx> {
    //     /// The lower bound of the range (inclusive).
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub start: Idx,
    //     /// The upper bound of the range (exclusive).
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub end: Idx,
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

    impl Index<Range<usize>> for A<T> {
	type Output = [T];

	fn index<'a>(&'a self, index: Range<usize>) -> &'a Self::Output {
	    assert!(index.start <= index.end);
	    assert!(index.end <= self.len());

	    unsafe {
		transmute::<Slice<T>, &'a [T]>(
		    Slice {
			data: self.as_ptr().offset(index.start as isize),
			len: index.end - index.start
		    }
		)
	    }
	}
    }

    type T = i32;

    #[test]
    fn range_test1() {
	let range: Range<usize> = Range { start: 100, end: 200 };
	let message: String = format!("{:?}", range);

	assert_eq!(message, "100..200".to_string());
    }

    #[test]
    fn range_test2() {
	let range: Range<usize> = 100..200;
	let message: String = format!("{:?}", range);

	assert_eq!(message, "100..200".to_string());
    }

    #[test]
    fn range_test3() {
	let a: A<T> = A { storage: [1, 2, 3, 4, 5, 6].repr()};
	let range: Range<usize> = 1..5;
	let slice: &[T] = &a[range];
	let repr: Slice<T> = slice.repr();

	assert_eq!(repr.len, 4);

	for i in 0..repr.len {
	    assert_eq!(slice[i], (i + 2) as T);
	}
    }
}
