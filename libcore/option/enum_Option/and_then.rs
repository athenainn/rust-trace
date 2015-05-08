#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    type T = u32;
    type U = u32;

    struct F;

    type Args = (T,);

    impl FnOnce<Args> for F {
	type Output = Option<U>;
	extern "rust-call" fn call_once(self, (arg,): Args) -> Self::Output {
	    Some::<U>(arg * arg)
	}
    }

    impl Clone for F {
	fn clone(&self) -> Self {
	    F
	}
    }

    impl Copy for F {}


    struct G;

    impl FnOnce<Args> for G {
	type Output = Option<U>;
	extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
	    None::<U>
	}
    }

    #[test]
    fn and_then_test1() {
	let x: Option<T> = Some(2);
	let sq: F = F;

	let y: Option<U> = x.and_then(sq).and_then(sq);

	assert_eq!(y, Some::<U>(16));
    }

    #[test]
    fn and_then_test2() {
	let x: Option<T> = Some(2);
	let sq: F = F;
	let nope: G = G;

	let y: Option<U> = x.and_then(sq).and_then(nope);

	assert_eq!(y, None::<U>);
    }

    #[test]
    fn and_then_test3() {
	let x: Option<T> = Some(2);
	let sq: F = F;
	let nope: G = G;

	let y: Option<U> = x.and_then(nope).and_then(sq);

	assert_eq!(y, None::<U>);
    }

    #[test]
    fn and_then_test4() {
	let x: Option<T> = None;
	let sq: F = F;

	let y: Option<U> = x.and_then(sq).and_then(sq);

	assert_eq!(y, None::<U>);
    }

    #[test]
    fn and_then_test5() {
	let x: Option<T> = None;
	let sq = |x: T| -> Option<U> { Some::<U>(x * x) };
	let nope = |_: T| -> Option<U> { None::<U> };

	let y: Option<U> = x.and_then(sq).and_then(nope);

	assert_eq!(y, None::<U>);
    }
}
