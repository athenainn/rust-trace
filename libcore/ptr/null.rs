#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::null;

    // pub fn null<T>() -> *const T { 0 as *const T }

    type T = i32;

    #[test]
    fn null_test1() {
	let null: *const T = null::<T>();

	assert_eq!(null, 0 as *const T);
    }
}
