#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
    use core::iter::Chain;

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

		// fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter> where
		//     Self: Sized, U: IntoIterator<Item=Self::Item>,
		// {
		//     Chain{a: self, b: other.into_iter(), flag: false}
		// }
	    }
	}
    }

    impl DoubleEndedIterator for A<T> {
	fn next_back(&mut self) -> Option<Self::Item> {
	    if self.begin < self.end {
		self.end = self.end.wrapping_sub(1);
		Some::<Self::Item>(self.end)
	    } else {
		None::<Self::Item>
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<A, B> DoubleEndedIterator for Chain<A, B> where
    //     A: DoubleEndedIterator,
    //     B: DoubleEndedIterator<Item=A::Item>,
    // {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<A::Item> {
    //         match self.b.next_back() {
    //             Some(x) => Some(x),
    //             None => self.a.next_back()
    //         }
    //     }
    // }

    #[test]
    fn next_back_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 20 };

	type U = A<T>;
	let mut chain: Chain<A<T>, U> = a.chain::<U>(other);

	for x in 0..20 {
	    let y: Option<T> = chain.next_back();
	    match y {
		Some(v) => { assert_eq!(v, 19 - x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(chain.next_back(), None::<T>);
    }
}
