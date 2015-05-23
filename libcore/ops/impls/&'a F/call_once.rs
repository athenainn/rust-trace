#![feature(core, unboxed_closures)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Fn;
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

    impl Fn<Args> for F {
	extern "rust-call" fn call(&self, (x,): Args) -> Self::Output {
	    x + 3
	}
    }

    #[test]
    fn call_once_test1() {
	let f: F = F;
	let f_ptr: &F = &f;
	let x: T = 68;
	let result: T = f_ptr.call_once((x,));

	assert_eq!(result, x + 3);
	assert_eq!(result, 71);
    }

    #[test]
    fn call_once_test2() {
	fn foo<F: FnOnce(T) -> T>(f: F, x: T) -> T {
	    f(x)
	}

	let f: F = F;
	let f_ptr: &F = &f;
	let x: T = 68;
	let result: T = foo(f_ptr, x);

	assert_eq!(result, x + 3);
	assert_eq!(result, 71);
    }
}
