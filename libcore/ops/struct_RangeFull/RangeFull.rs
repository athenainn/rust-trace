#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::RangeFull;
    use core::ops::Index;

    use core::raw::Slice;
    use core::raw::Repr;

    use core::intrinsics::transmute;

    struct A<T> {
	storage: Slice<T>
    }

    // pub struct RangeFull;

    // pub trait Index<Idx: ?Sized> {
    //     /// The returned type after indexing
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output: ?Sized;
    //
    //     /// The method for the indexing (`Foo[Bar]`) operation
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn index<'a>(&'a self, index: Idx) -> &'a Self::Output;
    // }

    impl Index<RangeFull> for A<T> {
	type Output = [T];

	fn index<'a>(&'a self, _: RangeFull) -> &'a Self::Output {
	    unsafe {
		transmute::<Slice<T>, &'a [T]>(self.storage)
	    }
	}
    }

    type T = i32;

    #[test]
    fn rangefull_test1() {
	let a: A<T> = A { storage: [1, 2, 3, 4, 5, 6].repr() };
	let rangefull: RangeFull = ..;
	let slice: &[T] = &a[rangefull];
	let repr: Slice<T> = slice.repr();

	assert_eq!(repr.len, 6);

	for i in 0..repr.len {
	    assert_eq!(slice[i], (i + 1) as T);
	}
    }
}
