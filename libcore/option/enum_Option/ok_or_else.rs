#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::FnOnce;

    type T = &'static str;
    type E = &'static str;
    type R = Result<T, E>;

    struct F;

    type Args = ();

    impl FnOnce<Args> for F {
	type Output = E;
	extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
	    "Error"
	}
    }

    #[test]
    fn ok_or_else_test1() {
	let x: Option<T> = None;
	let err: F = F;
	let a: R = x.ok_or_else(err);

	assert_eq!(a, Err("Error"));
    }

    #[test]
    fn ok_or_else_test2() {
	let x: Option<T> = Some("hello");
	let err: F = F;
	let a: R = x.ok_or_else(err);

	assert_eq!(a, Ok("hello"));
    }

    #[test]
    fn ok_or_else_test3() {
	let x: Option<T> = None;
	let err: F = F;
	let a: R = x.ok_or_else(|| "Error");

	assert_eq!(a, Err("Error"));
    }

    #[test]
    fn ok_or_else_test4() {
	let x: Option<T> = Some("hello");
	let err: F = F;
	let a: R = x.ok_or_else(|| "Error");

	assert_eq!(a, Ok("hello"));
    }
}
