#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Peekable;

    use core::clone::Clone;

    struct A<T> {
	begin: T,
	end: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self {
	    A {
		begin: self.begin,
		end: self.end
	    }
	}
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

		// fn peekable(self) -> Peekable<Self> where Self: Sized {
		//     Peekable{iter: self, peeked: None}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator + Clone> Clone for Peekable<I> where I::Item: Clone {
    //     fn clone(&self) -> Peekable<I> {
    //         Peekable {
    //             iter: self.iter.clone(),
    //             peeked: self.peeked.clone(),
    //         }
    //     }
    // }

    type I = A<T>;

    #[test]
    fn clone_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let peekable: Peekable<I> = a.peekable();
	let clone: Peekable<I> = peekable.clone();
    }
}
