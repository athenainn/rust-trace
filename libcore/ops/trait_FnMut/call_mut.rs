#![feature(core, unboxed_closures)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::FnMut;
    use core::ops::FnOnce;

    struct F;

    type T = i32;
    type Args = (T,);

    impl FnOnce<Args> for F {
	type Output = T;

	extern "rust-call" fn call_once(self, (x,): Args) -> Self::Output {
	    x * x
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (x,): Args) -> Self::Output {
	    x * x * x
	}
    }

    #[test]
    fn call_mut_test1() {
	let mut f: F = F;
	let x: T = 68;
	let result: T = f.call_mut((x,));

	assert_eq!(result, x * x * x);
	assert_eq!(result, 314432);
    }

    #[test]
    fn call_mut_test2() {
	let mut f: F = F;
	let x: T = 68;
	let result: T = f(x);

	assert_eq!(result, x * x * x);
	assert_eq!(result, 314432);
    }

    #[test]
    fn call_mut_test3() {
	fn foo<F: FnOnce(T) -> T>(f: F, x: T) -> T {
	    f(x)
	}

	let f: F = F;
	let x: T = 68;
	let result: T = foo(f, x);

	assert_eq!(result, x * x);
	assert_eq!(result, 4624);
    }

    #[test]
    fn call_mut_test4() {
	fn foo<F: FnMut(T) -> T>(mut f: F, x: T) -> T {
	    f(x)
	}

	let f: F = F;
	let x: T = 68;
	let result: T = foo(f, x);

	assert_eq!(result, x * x * x);
	assert_eq!(result, 314432);
    }
}
