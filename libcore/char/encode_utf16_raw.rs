#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::char::encode_utf16_raw;

    // pub fn encode_utf16_raw(mut ch: u32, dst: &mut [u16]) -> Option<usize> {
    //     // Marked #[inline] to allow llvm optimizing it away
    //     if (ch & 0xFFFF) == ch && !dst.is_empty() {
    //         // The BMP falls through (assuming non-surrogate, as it should)
    //         dst[0] = ch as u16;
    //         Some(1)
    //     } else if dst.len() >= 2 {
    //         // Supplementary planes break into surrogates.
    //         ch -= 0x1_0000;
    //         dst[0] = 0xD800 | ((ch >> 10) as u16);
    //         dst[1] = 0xDC00 | ((ch as u16) & 0x3FF);
    //         Some(2)
    //     } else {
    //         None
    //     }
    // }

    #[test]
    fn encode_utf16_test1() {
	let code: u32 = 0x1F37A;
	let dst: &mut [u16] = &mut [0, 0];
	let len: Option<usize> = encode_utf16_raw(code, dst);

	assert_eq!(len, Some::<usize>(2));
	assert_eq!(dst, &mut [0xd83c, 0xdf7a]);
    }
}
