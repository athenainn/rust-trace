#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::bytes::MutableByteVector;

    //     impl MutableByteVector for [u8] {
    //         #[inline]
    //         fn set_memory(&mut self, value: u8) {
    //             unsafe { ptr::write_bytes(self.as_mut_ptr(), value, self.len()) };
    //         }
    //     }

    #[test]
    fn set_memory_test1() {
	let slice: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
	let value: u8 = 0x0;

	slice.set_memory(value);

	assert_eq!(slice , &mut [0, 0, 0, 0, 0, 0]);
    }
}
