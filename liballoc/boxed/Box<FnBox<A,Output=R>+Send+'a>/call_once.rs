#![feature(core, alloc, unboxed_closures, fnbox)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use alloc::boxed::FnBox;

    use core::ops::FnOnce;

    use std::thread;

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

    // impl<'a,A,R> FnOnce<A> for Box<FnBox<A,Output=R>+'a> {
    //     type Output = R;
    //
    //     extern "rust-call" fn call_once(self, args: A) -> R {
    //         self.call_box(args)
    //     }
    // }

    // impl<'a,A,R> FnOnce<A> for Box<FnBox<A,Output=R>+Send+'a> {
    //     type Output = R;
    //
    //     extern "rust-call" fn call_once(self, args: A) -> R {
    //         self.call_box(args)
    //     }
    // }

    struct F;

    type T = i32;
    type A = (T,);

    impl FnOnce<A> for F {
	type Output = T;

	extern "rust-call" fn call_once(self, (x,): A) -> Self::Output {
	    x * x
	}
    }

    type R = T;

    #[test]
    fn call_box_test1() {
	let f: F = F;
	let b: Box<FnBox<A,Output=R> + Send> = Box::<F>::new(f);

	thread::spawn(|| {
	    let result: <F as FnOnce<A>>::Output = b(68);

	    assert_eq!(result, 68 * 68);
	});

	thread::sleep_ms(10);
    }
}
