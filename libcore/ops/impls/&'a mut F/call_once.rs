#![feature(core, unboxed_closures)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::FnMut;
    use core::ops::FnOnce;

    type T = i32;

    struct F;

    type Args = (T,);

    impl FnOnce<Args> for F {
	type Output = T;
	extern "rust-call" fn call_once(self, (x,): Args) -> Self::Output {
	    x + 1
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (x,): Args) -> Self::Output {
	    x + 2
	}
    }

    #[test]
    fn call_once_test1() {
	let mut f: F = F;
	let f_ptr: &mut F = &mut f;
	let x: T = 68;
	let result: T = f_ptr.call_once((x,));

	assert_eq!(result, x + 2);
	assert_eq!(result, 70);
    }

    #[test]
    fn call_once_test2() {
	fn foo<F: FnOnce(T) -> T>(f: F, x: T) -> T {
	    f(x)
	}

	let mut f: F = F;
	let f_ptr: &mut F = &mut f;
	let x: T = 68;
	let result: T = foo(f_ptr, x);

	assert_eq!(result, x + 2);
	assert_eq!(result, 70);
    }
}
