#![feature(core, unboxed_closures)]

extern crate core;

#[cfg(test)]
mod tests {
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

    #[test]
    fn call_once_test1() {
	let f: F = F;
	let x: T = 68;
	let result: T = f.call_once((x,));

	assert_eq!(result, x * x);
	assert_eq!(result, 4624);
    }

    #[test]
    fn call_once_test2() {
	let f: F = F;
	let x: T = 68;
	let result: T = f(x);

	assert_eq!(result, x * x);
	assert_eq!(result, 4624);
    }

    #[test]
    fn call_once_test3() {
	fn foo<F: FnOnce(T) -> T>(f: F, x: T) -> T {
	    f(x)
	}

	let f: F = F;
	let x: T = 68;
	let result: T = foo(f, x);

	assert_eq!(result, x * x);
	assert_eq!(result, 4624);
    }
}
