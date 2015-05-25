#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     pub fn is_null(self) -> bool where T: Sized {
    //         self == 0 as *const T
    //     }

    type T = i32;

    #[test]
    fn is_null_test1() {
	let ptr: *const T = 0 as *const T;
	let result: bool = ptr.is_null();

	assert_eq!(result, true);
    }

    #[test]
    fn is_null_test2() {
	let x: T = 68;
	let ptr: *const T = &x;
	let result: bool = ptr.is_null();

	assert_eq!(result, false);
    }
}
