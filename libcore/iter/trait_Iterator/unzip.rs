#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;

    use collections::vec;

    struct A<T> {
	begin: T,
	end: T
    }

    macro_rules! Iterator_impl {
	($T:ty) => {
	    impl Iterator for A<$T> {
		type Item = ($T, $T);

		fn next(&mut self) -> Option<Self::Item> {
		    if self.begin < self.end {
			let left: $T = self.begin;
			let right: $T = -left;
			self.begin = self.begin.wrapping_add(1);
			Some::<Self::Item>((left, right))
		    } else {
			None::<Self::Item>
		    }
		}

		// fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB) where
		//     FromA: Default + Extend<A>,
		//     FromB: Default + Extend<B>,
		//     Self: Sized + Iterator<Item=(A, B)>,
		// {
		//     struct SizeHint<A>(usize, Option<usize>, marker::PhantomData<A>);
		//     impl<A> Iterator for SizeHint<A> {
		//         type Item = A;
		//
		//         fn next(&mut self) -> Option<A> { None }
		//         fn size_hint(&self) -> (usize, Option<usize>) {
		//             (self.0, self.1)
		//         }
		//     }
		//
		//     let (lo, hi) = self.size_hint();
		//     let mut ts: FromA = Default::default();
		//     let mut us: FromB = Default::default();
		//
		//     ts.extend(SizeHint(lo, hi, marker::PhantomData));
		//     us.extend(SizeHint(lo, hi, marker::PhantomData));
		//
		//     for (t, u) in self {
		//         ts.extend(Some(t).into_iter());
		//         us.extend(Some(u).into_iter());
		//     }
		//
		//     (ts, us)
		// }
	    }
	}
    }

    struct B<T> {
	data: vec::Vec<T>
    }

    struct IntoIter<T> {
	iter: vec::IntoIter<T>
    }

    impl IntoIterator for B<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> IntoIter<T> {
	    IntoIter { iter: self.data.into_iter() }
	}
    }

    impl Iterator for IntoIter<T> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> { self.iter.next() }

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
    }

    impl Default for B<T> {
	fn default() -> Self {
	    B { data: vec!() }
	}
    }

    impl Extend<T> for B<T>
    {
	fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
	    self.data.extend(iter);
	}
    }

    type T = i32;
    Iterator_impl!(T);

    type AA = T;
    type BB = T;
    type FromA = B<T>;
    type FromB = B<T>;

    #[test]
    fn unzip_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };

	for x in 0..10 {
	    let y: Option<<A<T> as Iterator>::Item> = a.next();
	    assert_eq!(y, Some::<<A<T> as Iterator>::Item>((x, -x)));
	}

	assert_eq!(a.next(), None::<<A<T> as Iterator>::Item>);
    }

    #[test]
    fn unzip_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let (left, right): (FromA, FromB) = a.unzip::<AA, BB, FromA, FromB>();
	let mut left_iter: IntoIter<T> = left.into_iter();
	let mut right_iter: IntoIter<T> = right.into_iter();

	for x in 0..10 {
	    let y: Option<<FromA as IntoIterator>::Item> = left_iter.next();
	    let z: Option<<FromB as IntoIterator>::Item> = right_iter.next();
	    match (y, z) {
		(Some(u), Some(v)) => assert_eq!((u, v), (x, -x)),
		_ => assert!(false)
	    }
	}

	assert_eq!(left_iter.next(), None::<<FromA as IntoIterator>::Item>);
	assert_eq!(right_iter.next(), None::<<FromB as IntoIterator>::Item>);
    }
}
