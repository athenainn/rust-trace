#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::TakeWhile;

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

		// fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> where
		//     Self: Sized, P: FnMut(&Self::Item) -> bool,
		// {
		//     TakeWhile{iter: self, flag: false, predicate: predicate}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    struct P;

    type Item = T;
    type Args<'a> = (&'a Item,);

    impl<'a> FnOnce<Args<'a>> for P {
	type Output = bool;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    match item {
		_ if *item >= 0 => true,
		_ => false
	    }
	}
    }

    impl<'a> FnMut<Args<'a>> for P {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		_ if *item >= 0 => true,
		_ => false
	    }
	}
    }

    #[test]
    fn take_while_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let predicate: P = P;
	let mut take_while: TakeWhile<A<T>, P> = a.take_while::<P>(predicate);

	for x in 0..10 {
	    let y: Option<Item> = take_while.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(take_while.next(), None::<Item>);
    }

    #[test]
    fn take_while_test2() {
	let a: A<T> = A { begin: -1, end: 10 };
	let predicate: P = P;
	let mut take_while: TakeWhile<A<T>, P> = a.take_while::<P>(predicate);

	assert_eq!(take_while.next(), None::<Item>);
    }

    #[test]
    fn take_while_test3() {
	let a: A<T> = A { begin: 1, end: 6 };
	let mut take_while = a.take_while(|&x| x < 3);

	assert_eq!(take_while.next(), Some::<Item>(1));
	assert_eq!(take_while.next(), Some::<Item>(2));
	assert_eq!(take_while.next(), None::<Item>);
    }
}
