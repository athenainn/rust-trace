#![feature(core, collections, unboxed_closures)]
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
		type Item = $T;

		fn next(&mut self) -> Option<Self::Item> {
		    if self.begin < self.end {
			let result = self.begin;
			self.begin = self.begin.wrapping_add(1);
			Some::<Self::Item>(result)
		    } else {
			None::<Self::Item>
		    }
		}

		// fn partition<B, F>(self, mut f: F) -> (B, B) where
		//     Self: Sized,
		//     B: Default + Extend<Self::Item>,
		//     F: FnMut(&Self::Item) -> bool
		// {
		//     let mut left: B = Default::default();
		//     let mut right: B = Default::default();
		//
		//     for x in self {
		//         if f(&x) {
		//             left.extend(Some(x).into_iter())
		//         } else {
		//             right.extend(Some(x).into_iter())
		//         }
		//     }
		//
		//     (left, right)
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

    struct F;

    type Item = T;
    type Args<'a> = (&'a Item,);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = bool;
	extern "rust-call" fn call_once(self, (&item,): Args) -> Self::Output {
	    item % 2 == 0
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (&item,): Args) -> Self::Output {
	    item % 2 == 0
	}
    }

    #[test]
    fn partition_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let (left, right): (B<T>, B<T>) = a.partition(f); // invoke B<T>::extend()

	let mut left_iter: IntoIter<T> = left.into_iter();
	let mut right_iter: IntoIter<T> = right.into_iter();

	for x in 0..5 {
	    let y: Option<Item> = left_iter.next();
	    match y {
		Some(v) => { assert_eq!(v, 2 * x); }
		None => { assert!(false); }
	    }
	}

	for x in 0..5 {
	    let y: Option<Item> = right_iter.next();
	    match y {
		Some(v) => { assert_eq!(v, 2 * x + 1); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(left_iter.next(), None::<Item>);
	assert_eq!(right_iter.next(), None::<Item>);
    }
}
