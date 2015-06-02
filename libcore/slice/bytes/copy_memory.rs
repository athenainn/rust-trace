#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::bytes::copy_memory;

    //     pub fn copy_memory(src: &[u8], dst: &mut [u8]) {
    //         let len_src = src.len();
    //         assert!(dst.len() >= len_src);
    //         // `dst` is unaliasable, so we know statically it doesn't overlap
    //         // with `src`.
    //         unsafe {
    //             ptr::copy_nonoverlapping(src.as_ptr(),
    //                                      dst.as_mut_ptr(),
    //                                      len_src);
    //         }
    //     }

    #[test]
    #[should_panic]
    fn copy_memory_test1() {
	let src: &[u8] = &[1u8, 2, 3, 4, 5, 6];
	let dst: &mut [u8] = &mut [7, 8, 9];

	copy_memory(src, dst); // panicked at 'assertion failed: dst.len() >= len_src'
    }

    #[test]
    fn copy_memory_test2() {
	let src: &[u8] = &[1u8, 2, 3, 4, 5, 6];
	let dst: &mut [u8] = &mut [7, 8, 9, 10, 11, 12];

	copy_memory(src, dst);

	assert_eq!(src, dst);
    }

    #[test]
    fn copy_memory_test3() {
	let src: &[u8] = &[1u8, 2, 3, 4, 5, 6];
	let dst: &mut [u8] = &mut [7, 8, 9, 10, 11, 12, 13];

	copy_memory(src, dst);

	assert_eq!(dst, &mut[1, 2, 3, 4, 5, 6, 13]);
    }
}
