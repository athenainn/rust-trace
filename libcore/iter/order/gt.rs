#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::order::gt;

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

    //     pub fn gt<L: Iterator, R: Iterator>(mut a: L, mut b: R) -> bool where
    //         L::Item: PartialOrd<R::Item>,
    //     {
    //         loop {
    //             match (a.next(), b.next()) {
    //                 (None, None) => return false,
    //                 (None, _   ) => return false,
    //                 (_   , None) => return true,
    //                 (Some(x), Some(y)) => if x.ne(&y) { return x.gt(&y) },
    //             }
    //         }
    //     }

    type T = i32;
    Iterator_impl!(T);

    type L = A<T>;
    type R = A<T>;

    #[test]
    fn gt_test1() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 0, end: 10 };
	let result: bool = gt::<L, R>(a, b);

	assert_eq!(result, false);
    }

    #[test]
    fn gt_test2() {
	let a: L = L { begin: 0, end: 9 };
	let b: R = R { begin: 0, end: 10 };
	let result: bool = gt::<L, R>(a, b);

	assert_eq!(result, false);
    }

    #[test]
    fn gt_test3() {
	let a: L = L { begin: 0, end: 11 };
	let b: R = R { begin: 0, end: 10 };
	let result: bool = gt::<L, R>(a, b);

	assert_eq!(result, true);
    }

    #[test]
    fn gt_test4() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 10, end: 20 };
	let result: bool = gt::<L, R>(a, b);

	assert_eq!(result, false);
    }

    #[test]
    fn gt_test5() {
	let a: L = L { begin: 10, end: 20 };
	let b: R = R { begin: 0, end: 10 };
	let result: bool = gt::<L, R>(a, b);

	assert_eq!(result, true);
    }
}
