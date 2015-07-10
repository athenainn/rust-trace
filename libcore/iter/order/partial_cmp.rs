#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::order::partial_cmp;

    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialOrd;

    struct A<T: PartialOrd> {
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
	    }
	}
    }

    //     pub fn partial_cmp<L: Iterator, R: Iterator>(mut a: L, mut b: R) -> Option<cmp::Ordering> where
    //         L::Item: PartialOrd<R::Item>
    //     {
    //         loop {
    //             match (a.next(), b.next()) {
    //                 (None, None) => return Some(Equal),
    //                 (None, _   ) => return Some(Less),
    //                 (_   , None) => return Some(Greater),
    //                 (Some(x), Some(y)) => match x.partial_cmp(&y) {
    //                     Some(Equal) => (),
    //                     non_eq => return non_eq,
    //                 },
    //             }
    //         }
    //     }

    type T = i32;
    Iterator_impl!(T);

    type L = A<T>;
    type R = A<T>;

    #[test]
    fn partial_cmp_test1() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 0, end: 10 };
	let result: Option<Ordering> = partial_cmp::<L, R>(a, b);

	assert_eq!(result.unwrap(), Equal);
    }

    #[test]
    fn partial_cmp_test2() {
	let a: L = L { begin: 0, end: 9 };
	let b: R = R { begin: 0, end: 10 };
	let result: Option<Ordering> = partial_cmp::<L, R>(a, b);

	assert_eq!(result.unwrap(), Less);
    }

    #[test]
    fn partial_cmp_test3() {
	let a: L = L { begin: 0, end: 11 };
	let b: R = R { begin: 0, end: 10 };
	let result: Option<Ordering> = partial_cmp::<L, R>(a, b);

	assert_eq!(result.unwrap(), Greater);
    }

    #[test]
    fn partial_cmp_test4() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 10, end: 20 };
	let result: Option<Ordering> = partial_cmp::<L, R>(a, b);

	assert_eq!(result.unwrap(), Less);
    }

    #[test]
    fn partial_cmp_test5() {
	let a: L = L { begin: 10, end: 20 };
	let b: R = R { begin: 0, end: 10 };
	let result: Option<Ordering> = partial_cmp::<L, R>(a, b);

	assert_eq!(result.unwrap(), Greater);
    }
}
