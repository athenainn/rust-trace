#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    type T = &'static str;

    struct F;

    type Args = ();

    impl FnOnce<Args> for F {
	type Output = Option<T>;
	extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
	    Some::<T>("vikings")
	}
    }

    struct G;

    impl FnOnce<Args> for G {
	type Output = Option<T>;
	extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
	    None::<T>
	}
    }

    #[test]
    fn or_then_test1() {
	let x: Option<T> = Some("barbarians");
	let f: F = F;

	let y: Option<T> = x.or_else(f);
	assert_eq!(y, Some::<T>("barbarians"));
    }

    #[test]
    fn or_then_test2() {
	let x: Option<T> = None::<T>;
	let f: F = F;

	let y: Option<T> = x.or_else(f);
	assert_eq!(y, Some::<T>("vikings"));
    }

    #[test]
    fn or_then_test3() {
	let x: Option<T> = None::<T>;
	let g: G = G;

	let y: Option<T> = x.or_else(g);
	assert_eq!(y, None::<T>);
    }
}
