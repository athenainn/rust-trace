#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::FromIterator;

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

		// fn collect<B: FromIterator<Self::Item>>(self) -> B where Self: Sized {
		//     FromIterator::from_iter(self)
		// }
	    }
	}
    }

    type T = i32;
    type B = A<T>;

    macro_rules! FromIterator_impl {
	($T:ty) => {
	    impl FromIterator<$T> for A<$T> {
		fn from_iter<I: IntoIterator<Item=$T>>(iter: I) -> Self {
		    let mut it: <I as IntoIterator>::IntoIter = iter.into_iter();

		    if let Some(begin) = it.next() {
			let last: $T = match it.last() { Some(v) => v, None => begin};
			A { begin: begin, end: last + 1 }
		    } else {
			A { begin: 0, end: 0 }
		    }
		}
	    }
	}
    }

    Iterator_impl!(T);
    FromIterator_impl!(T);

    type Item = T;

    #[test]
    fn collect_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut collect: B = a.collect::<B>();

	for x in 0..10 {
	    let y: Option<Item> = collect.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(collect.next(), None::<Item>);
    }
}
