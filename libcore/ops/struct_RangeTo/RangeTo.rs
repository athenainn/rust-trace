#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::RangeTo;
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

    // pub struct RangeTo<Idx> {
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

    impl Index<RangeTo<usize>> for A<T> {
	type Output = [T];

	fn index<'a>(&'a self, index: RangeTo<usize>) -> &'a Self::Output {
	    assert!(index.end <= self.len());

	    unsafe {
		transmute::<Slice<T>, &'a [T]>(
		    Slice {
			data: self.as_ptr(),
			len: index.end
		    }
		)
	    }
	}
    }

    type T = i32;

    #[test]
    fn rangeto_test1() {
	let rangeto: RangeTo<T> = RangeTo { end: 200 };
	let message: String = format!("{:?}", rangeto);

	assert_eq!(message, "..200".to_string());
    }

    #[test]
    fn rangeto_test2() {
	let rangeto: RangeTo<T> = ..200;
	let message: String = format!("{:?}", rangeto);

	assert_eq!(message, "..200".to_string());
    }

    #[test]
    fn rangeto_test3() {
	let a: A<T> = A { storage: [1, 2, 3, 4, 5, 6].repr() };
	let range: RangeTo<usize> = ..4;
	let slice: &[T] = &a[range];
	let repr: Slice<T> = slice.repr();

	assert_eq!(repr.len, 4);

	for i in 0..repr.len {
	    assert_eq!(slice[i], (i + 1) as T);
	}
    }
}
