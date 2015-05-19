#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::order::equals;

    use core::cmp::Eq;

    struct A<T: Eq> {
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

    //     pub fn equals<A, L, R>(mut a: L, mut b: R) -> bool where
    //         A: Eq,
    //         L: Iterator<Item=A>,
    //         R: Iterator<Item=A>,
    //     {
    //         loop {
    //             match (a.next(), b.next()) {
    //                 (None, None) => return true,
    //                 (None, _) | (_, None) => return false,
    //                 (Some(x), Some(y)) => if x != y { return false },
    //             }
    //         }
    //     }

    type T = i32;
    Iterator_impl!(T);

    type AA = T;
    type L = A<T>;
    type R = A<T>;

    #[test]
    fn equals_test1() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 0, end: 10 };
	let result: bool = equals::<AA, L, R>(a, b);

	assert_eq!(result, true);
    }

    #[test]
    fn equals_test2() {
	let a: L = L { begin: 0, end: 10 };
	let b: R = R { begin: 0, end: 11 };
	let result: bool = equals::<AA, L, R>(a, b);

	assert_eq!(result, false);
    }

    #[test]
    fn equals_test3() {
	let a: L = L { begin: 0, end: 0 };
	let b: R = R { begin: 0, end: 0 };
	let result: bool = equals::<AA, L, R>(a, b);

	assert_eq!(result, true);
    }
}
