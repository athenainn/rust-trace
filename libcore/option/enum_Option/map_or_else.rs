#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::FnOnce;

    type T = i32;
    type U = i32;

    struct F;

    type FArgs = (T,);

    impl FnOnce<FArgs> for F {
	type Output = U;
	extern "rust-call" fn call_once(self, (arg,): FArgs) -> Self::Output {
	    -arg
	}
    }

    struct D;

    type DArgs = ();

    impl FnOnce<DArgs> for D {
	type Output = U;
	extern "rust-call" fn call_once(self, _: DArgs) -> Self::Output {
	    -1
	}
    }

    #[test]
    fn map_or_else_test1() {
	let x: Option<T> = None;
	let def: D = D;
	let f: F = F;
	let a: U = x.map_or_else(def, f);

	assert_eq!(a, -1);
    }

    #[test]
    fn map_or_else_test2() {
	let x: Option<T> = Some(68);
	let def: D = D;
	let f: F = F;
	let a: U = x.map_or_else(def, f);

	assert_eq!(a, -68);
    }

    #[test]
    fn map_or_else_test3() {
	let x: Option<T> = None;
	let def: D = D;
	let a: U = x.map_or_else(def, |x| -x);

	assert_eq!(a, -1);
    }

    #[test]
    fn map_or_else_test4() {
	let x: Option<T> = Some(68);
	let def: D = D;
	let a: U = x.map_or_else(def, |x| -x);

	assert_eq!(a, -68);
    }

    #[test]
    fn map_or_else_test5() {
	let x: Option<T> = None;
	let a: U = x.map_or_else(|| -1, |x| -x);

	assert_eq!(a, -1);
    }

    #[test]
    fn map_or_else_test6() {
	let x: Option<T> = Some(68);
	let a: U = x.map_or_else(|| -1, |x| -x);

	assert_eq!(a, -68);
    }
}
