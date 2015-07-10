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
    fn call_test1() {
	let f: F = F;
	let x: T = 68;
	let result: T = f.call((x,));

	assert_eq!(result, x + 3);
	assert_eq!(result, 71);
    }

    #[test]
    fn call_test2() {
	let f: F = F;
	let x: T = 68;
	let result: T = f(x);

	assert_eq!(result, x + 3);
	assert_eq!(result, 71);
    }

    #[test]
    fn call_test3() {
	fn foo<F: FnOnce(T) -> T>(f: F, x: T) -> T {
	    f(x)
	}

	let f: F = F;
	let x: T = 68;
	let result: T = foo(f, x);

	assert_eq!(result, x + 1);
	assert_eq!(result, 69);
    }

    #[test]
    fn call_test4() {
	fn foo<F: FnMut(T) -> T>(mut f: F, x: T) -> T {
	    f(x)
	}

	let f: F = F;
	let x: T = 68;
	let result: T = foo(f, x);

	assert_eq!(result, x + 2);
	assert_eq!(result, 70);
    }

    #[test]
    fn call_test5() {
	fn foo<F: Fn(T) -> T>(f: F, x: T) -> T {
	    f(x)
	}

	let f: F = F;
	let x: T = 68;
	let result: T = foo(f, x);

	assert_eq!(result, x + 3);
	assert_eq!(result, 71);
    }
}
