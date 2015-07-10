#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::drop;

    // pub fn drop<T>(_x: T) { }

    static mut dtor_count: usize = 0;

    struct A;

    impl Drop for A {
	fn drop(&mut self) {
	    unsafe { dtor_count += 1 };
	}
    }

    type T = i32;

    #[test]
    fn drop_test1() {
	let x: A = A;

	drop::<A>(x);

	assert_eq!(unsafe { dtor_count }, 1);
    }
}
