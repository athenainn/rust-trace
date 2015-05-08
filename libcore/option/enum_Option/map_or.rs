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
    fn map_or_test1() {
	let x: Option<T> = None;
	let def: U = -1;
	let f: F = F;
	let a: U = x.map_or(def, f);

	assert_eq!(a, def);
    }

    #[test]
    fn map_or_test2() {
	let x: Option<T> = Some(68);
	let def: U = -1;
	let f: F = F;
	let a: U = x.map_or(def, f);

	assert_eq!(a, -68);
    }

    #[test]
    fn map_or_test3() {
	let x: Option<T> = None;
	let def: U = -1;
	let a: U = x.map_or(def, |x| -x);

	assert_eq!(a, def);
    }

    #[test]
    fn map_or_test4() {
	let x: Option<T> = None;
	let a: U = x.map_or(def, |x| -x);

	assert_eq!(a, def);
    }
}
