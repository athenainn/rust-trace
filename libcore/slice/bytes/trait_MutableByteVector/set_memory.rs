#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::bytes::MutableByteVector;

    use core::ptr::write_bytes;

    use core::mem::size_of;

    struct A<T> {
	value: T
    }

    impl MutableByteVector for A<T> {
	fn set_memory(&mut self, value: u8) {
	    unsafe {
		write_bytes(self, value, size_of::<T>());
	    }
	}
    }

    type T = u32;

    #[test]
    fn set_memory_test1() {
	let mut a: A<T> = A { value: 0xffffffff };
	let value: u8 = 0x0;

	a.set_memory(value);

	assert_eq!(a.value , 0);
    }
}
