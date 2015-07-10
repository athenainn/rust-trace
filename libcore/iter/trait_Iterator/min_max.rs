#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::MinMaxResult::{self, NoElements, OneElement, MinMax};

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

		// fn min_max(mut self) -> MinMaxResult<Self::Item> where Self: Sized, Self::Item: Ord
		// {
		//     let (mut min, mut max) = match self.next() {
		//         None => return NoElements,
		//         Some(x) => {
		//             match self.next() {
		//                 None => return OneElement(x),
		//                 Some(y) => if x <= y {(x, y)} else {(y, x)}
		//             }
		//         }
		//     };
		//
		//     loop {
		//         // `first` and `second` are the two next elements we want to look
		//         // at.  We first compare `first` and `second` (#1). The smaller one
		//         // is then compared to current minimum (#2). The larger one is
		//         // compared to current maximum (#3). This way we do 3 comparisons
		//         // for 2 elements.
		//         let first = match self.next() {
		//             None => break,
		//             Some(x) => x
		//         };
		//         let second = match self.next() {
		//             None => {
		//                 if first < min {
		//                     min = first;
		//                 } else if first >= max {
		//                     max = first;
		//                 }
		//                 break;
		//             }
		//             Some(x) => x
		//         };
		//         if first <= second {
		//             if first < min { min = first }
		//             if second >= max { max = second }
		//         } else {
		//             if second < min { min = second }
		//             if first >= max { max = first }
		//         }
		//     }
		//
		//     MinMax(min, max)
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    type Item = T;

    #[test]
    fn min_max_test1() {
	let a: A<T> = A { begin: 1, end: 1 };
	let min_max: MinMaxResult<Item> = a.min_max();

	assert_eq!(min_max, NoElements::<Item>);
    }

    #[test]
    fn min_max_test2() {
	let a: A<T> = A { begin: 2, end: 3 };
	let min_max: MinMaxResult<Item> = a.min_max();

	assert_eq!(min_max, OneElement::<Item>(2));
    }

    #[test]
    fn min_max_test3() {
	let a: A<T> = A { begin: 3, end: 9 };
	let min_max: MinMaxResult<Item> = a.min_max();

	assert_eq!(min_max, MinMax::<Item>(3, 8));
    }
}
