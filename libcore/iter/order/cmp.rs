#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::order::cmp;

    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::Ord;

    struct A<T: Ord> {
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

    //     pub fn cmp<A, L, R>(mut a: L, mut b: R) -> cmp::Ordering where
    //         A: Ord,
    //         L: Iterator<Item=A>,
    //         R: Iterator<Item=A>,
    //     {
    //         loop {
    //             match (a.next(), b.next()) {
    //                 (None, None) => return Equal,
    //                 (None, _   ) => return Less,
    //                 (_   , None) => return Greater,
    //                 (Some(x), Some(y)) => match x.cmp(&y) {
    //                     Equal => (),
    //                     non_eq => return non_eq,
    //                 },
    //             }
    //         }
    //     }

    type T = i32;
    Iterator_impl!(T);

    type AA = T;
    type L = A<T>;
    type R = A<T>;

    #[test]
    fn cmp_test1() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 0, end: 10 };
	let result: Ordering = cmp::<AA, L, R>(a, b);

	assert_eq!(result, Equal);
    }

    #[test]
    fn cmp_test2() {
	let a: L = L { begin: 0, end: 9 };
	let b: R = R { begin: 0, end: 10 };
	let result: Ordering = cmp::<AA, L, R>(a, b);

	assert_eq!(result, Less);
    }

    #[test]
    fn cmp_test3() {
	let a: L = L { begin: 0, end: 11 };
	let b: R = R { begin: 0, end: 10 };
	let result: Ordering = cmp::<AA, L, R>(a, b);

	assert_eq!(result, Greater);
    }

    #[test]
    fn cmp_test4() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 10, end: 20 };
	let result: Ordering = cmp::<AA, L, R>(a, b);

	assert_eq!(result, Less);
    }

    #[test]
    fn cmp_test5() {
	let a: L = L { begin: 10, end: 20 };
	let b: R = R { begin: 0, end: 10 };
	let result: Ordering = cmp::<AA, L, R>(a, b);

	assert_eq!(result, Greater);
    }
}
