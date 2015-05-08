#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::FnOnce;

    struct F;

    type T = i32;
    type U = i32;

    type Args = (T,);

    impl FnOnce<Args> for F {
	type Output = U;
	extern "rust-call" fn call_once(self, (arg,): Args) -> Self::Output {
	    -arg
	}
    }

    #[test]
    fn map_test1() {
	let x: Option<T> = Some(68);
	let f: F = F;
	let a: Option<U> = x.map(f);

	assert_eq!(a.unwrap(), -68);
    }

    #[test]
    fn map_test2() {
	let x: Option<T> = None;
	let f: F = F;
	let a: Option<U> = x.map(f);

	assert_eq!(a, None::<U>);
    }

    #[test]
    fn map_test3() {
	let x: Option<T> = Some(68);
	let a: Option<U> = x.map(|x| -x);

	assert_eq!(a.unwrap(), -68);
    }

    #[test]
    fn map_test4() {
	let x: Option<T> = None;
	let a: Option<U> = x.map(|x| -x);

	assert_eq!(a, None::<U>);
    }
}
