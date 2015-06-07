#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::char::encode_utf8_raw;

    // const TAG_CONT: u8    = 0b1000_0000;
    // const TAG_TWO_B: u8   = 0b1100_0000;
    // const TAG_THREE_B: u8 = 0b1110_0000;
    // const TAG_FOUR_B: u8  = 0b1111_0000;
    // const MAX_ONE_B: u32   =     0x80;
    // const MAX_TWO_B: u32   =    0x800;
    // const MAX_THREE_B: u32 =  0x10000;

    // pub fn encode_utf8_raw(code: u32, dst: &mut [u8]) -> Option<usize> {
    //     // Marked #[inline] to allow llvm optimizing it away
    //     if code < MAX_ONE_B && !dst.is_empty() {
    //         dst[0] = code as u8;
    //         Some(1)
    //     } else if code < MAX_TWO_B && dst.len() >= 2 {
    //         dst[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
    //         dst[1] = (code & 0x3F) as u8 | TAG_CONT;
    //         Some(2)
    //     } else if code < MAX_THREE_B && dst.len() >= 3  {
    //         dst[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
    //         dst[1] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
    //         dst[2] = (code & 0x3F) as u8 | TAG_CONT;
    //         Some(3)
    //     } else if dst.len() >= 4 {
    //         dst[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
    //         dst[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
    //         dst[2] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
    //         dst[3] = (code & 0x3F) as u8 | TAG_CONT;
    //         Some(4)
    //     } else {
    //         None
    //     }
    // }

    #[test]
    fn encode_utf8_raw_test1() {
	let code: u32 = 0x1F37A;
	let dst: &mut [u8] = &mut [0, 0, 0, 0];
	let len: Option<usize> = encode_utf8_raw(code, dst);

	assert_eq!(len, Some::<usize>(4));
	assert_eq!(dst, &mut [0xf0, 0x9f, 0x8d, 0xba]);
    }
}
