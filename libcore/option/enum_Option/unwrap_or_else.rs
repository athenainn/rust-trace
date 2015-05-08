#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::FnOnce;

    struct F;

    type T = i32;

    type Args = ();

    impl FnOnce<Args> for F {
	type Output = T;
	extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
	    -1
	}
    }

    #[test]
    fn unwrap_or_else_test1() {
	let x: Option<T> = Some(68);
	let f: F = F;
	let y: T = x.unwrap_or_else(f);

	assert_eq!(y, 68);
    }

    #[test]
    fn unwrap_or_else_test2() {
	let x: Option<T> = None;
	let f: F = F;
	let y: T = x.unwrap_or_else(f);

	assert_eq!(y, -1);
    }

    #[test]
    fn unwrap_or_else_test3() {
	let x: Option<T> = Some(68);
	let y: T = x.unwrap_or_else(|| -1);

	assert_eq!(y, 68);
    }

    #[test]
    fn unwrap_or_else_test4() {
	let x: Option<T> = None;
	let y: T = x.unwrap_or_else(|| -1);

	assert_eq!(y, -1);
    }
}
