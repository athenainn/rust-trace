#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     pub fn is_null(self) -> bool where T: Sized {
    //         self == 0 as *mut T
    //     }

    type T = i32;

    #[test]
    fn is_null_test1() {
	let ptr: *mut T = 0 as *mut T;
	let result: bool = ptr.is_null();

	assert_eq!(result, true);
    }

    #[test]
    fn is_null_test2() {
	let mut x: T = 68;
	let ptr: *mut T = &mut x;
	let result: bool = ptr.is_null();

	assert_eq!(result, false);
    }
}
