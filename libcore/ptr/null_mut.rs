#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::null_mut;

    // pub fn null_mut<T>() -> *mut T { 0 as *mut T }

    type T = i32;

    #[test]
    fn null_mut_test1() {
	let null: *mut T = null_mut::<T>();

	assert_eq!(null, 0 as *mut T);
    }
}
