#![feature(core, alloc, unboxed_closures, fnbox)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use alloc::boxed::FnBox;

    use core::ops::FnOnce;

    // pub struct Box<T>(Unique<T>);

    // impl<T> Box<T> {
    //     /// Allocates memory on the heap and then moves `x` into it.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Box::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline(always)]
    //     pub fn new(x: T) -> Box<T> {
    //         box x
    //     }
    // }

    // pub trait FnBox<A> {
    //     type Output;
    //
    //     fn call_box(self: Box<Self>, args: A) -> Self::Output;
    // }

    // impl<A,F> FnBox<A> for F
    //     where F: FnOnce<A>
    // {
    //     type Output = F::Output;
    //
    //     fn call_box(self: Box<F>, args: A) -> F::Output {
    //         self.call_once(args)
    //     }
    // }

    struct F;

    type T = i32;
    type A = (T, T);

    impl FnOnce<A> for F {
	type Output = T;

	extern "rust-call" fn call_once(self, (x, y): A) -> Self::Output {
	    x + y
	}
    }

    #[test]
    fn call_box_test1() {
	let f: F = F;
	let args: A = (68, 500);
	let b: Box<F> = Box::<F>::new(f);

	let result: <F as FnOnce<A>>::Output = b.call_box(args);

	assert_eq!(result, 568);
    }
}
