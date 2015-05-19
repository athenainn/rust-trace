#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::IntoIterator;
    use core::iter::Extend;

    use collections::vec;

    struct A<T> {
	data: vec::Vec<T>
    }

    struct IntoIter<T> {
	iter: vec::IntoIter<T>
    }

    impl<T> IntoIterator for A<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> IntoIter<T> {
	    IntoIter { iter: self.data.into_iter() }
	}
    }

    impl<T> Iterator for IntoIter<T> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> { self.iter.next() }

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
    }

    impl<T> Extend<T> for A<T>
    {
	fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
	    self.data.extend(iter);
	}
    }

    type T = i32;
    type Item = T;

    #[test]
    fn extend_test1() {
	let mut a: A<T> = A { data: vec!(1, 2, 3) };
	let v: Vec<T> = vec!(4, 5, 6);

	a.extend(v);

	let mut it: IntoIter<T> = a.into_iter();

	for x in 1..7 {
	    let y: Option<Item> = it.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(it.next(), None::<Item>);
    }
}
