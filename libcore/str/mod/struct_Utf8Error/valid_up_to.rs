#![feature(core, utf8_error)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::from_utf8;
    use core::str::Utf8Error;

    // impl Utf8Error {
    //     /// Returns the index in the given string up to which valid UTF-8 was
    //     /// verified.
    //     ///
    //     /// Starting at the index provided, but not necessarily at it precisely, an
    //     /// invalid UTF-8 encoding sequence was found.
    //     #[unstable(feature = "utf8_error", reason = "method just added")]
    //     pub fn valid_up_to(&self) -> usize { self.valid_up_to }
    // }

    // impl Utf8Error {
    //     /// Returns the index in the given string up to which valid UTF-8 was
    //     /// verified.
    //     ///
    //     /// Starting at the index provided, but not necessarily at it precisely, an
    //     /// invalid UTF-8 encoding sequence was found.
    //     #[unstable(feature = "utf8_error", reason = "method just added")]
    //     pub fn valid_up_to(&self) -> usize { self.valid_up_to }
    // }

    // pub fn from_utf8(v: &[u8]) -> Result<&str, Utf8Error> {
    //     try!(run_utf8_validation_iterator(&mut v.iter()));
    //     Ok(unsafe { from_utf8_unchecked(v) })
    // }

    // fn run_utf8_validation_iterator(iter: &mut slice::Iter<u8>)
    //                                 -> Result<(), Utf8Error> {
    //     let whole = iter.as_slice();
    //     loop {
    //         // save the current thing we're pointing at.
    //         let old = iter.clone();
    //
    //         // restore the iterator we had at the start of this codepoint.
    //         macro_rules! err { () => {{
    //             *iter = old.clone();
    //             return Err(Utf8Error {
    //                 valid_up_to: whole.len() - iter.as_slice().len()
    //             })
    //         }}}
    //
    //         macro_rules! next { () => {
    //             match iter.next() {
    //                 Some(a) => *a,
    //                 // we needed data, but there was none: error!
    //                 None => err!(),
    //             }
    //         }}
    //
    //         let first = match iter.next() {
    //             Some(&b) => b,
    //             // we're at the end of the iterator and a codepoint
    //             // boundary at the same time, so this string is valid.
    //             None => return Ok(())
    //         };
    //
    //         // ASCII characters are always valid, so only large
    //         // bytes need more examination.
    //         if first >= 128 {
    //             let w = UTF8_CHAR_WIDTH[first as usize];
    //             let second = next!();
    //             // 2-byte encoding is for codepoints  \u{0080} to  \u{07ff}
    //             //        first  C2 80        last DF BF
    //             // 3-byte encoding is for codepoints  \u{0800} to  \u{ffff}
    //             //        first  E0 A0 80     last EF BF BF
    //             //   excluding surrogates codepoints  \u{d800} to  \u{dfff}
    //             //               ED A0 80 to       ED BF BF
    //             // 4-byte encoding is for codepoints \u{1000}0 to \u{10ff}ff
    //             //        first  F0 90 80 80  last F4 8F BF BF
    //             //
    //             // Use the UTF-8 syntax from the RFC
    //             //
    //             // https://tools.ietf.org/html/rfc3629
    //             // UTF8-1      = %x00-7F
    //             // UTF8-2      = %xC2-DF UTF8-tail
    //             // UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
    //             //               %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
    //             // UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
    //             //               %xF4 %x80-8F 2( UTF8-tail )
    //             match w {
    //                 2 => if second & !CONT_MASK != TAG_CONT_U8 {err!()},
    //                 3 => {
    //                     match (first, second, next!() & !CONT_MASK) {
    //                         (0xE0         , 0xA0 ... 0xBF, TAG_CONT_U8) |
    //                         (0xE1 ... 0xEC, 0x80 ... 0xBF, TAG_CONT_U8) |
    //                         (0xED         , 0x80 ... 0x9F, TAG_CONT_U8) |
    //                         (0xEE ... 0xEF, 0x80 ... 0xBF, TAG_CONT_U8) => {}
    //                         _ => err!()
    //                     }
    //                 }
    //                 4 => {
    //                     match (first, second, next!() & !CONT_MASK, next!() & !CONT_MASK) {
    //                         (0xF0         , 0x90 ... 0xBF, TAG_CONT_U8, TAG_CONT_U8) |
    //                         (0xF1 ... 0xF3, 0x80 ... 0xBF, TAG_CONT_U8, TAG_CONT_U8) |
    //                         (0xF4         , 0x80 ... 0x8F, TAG_CONT_U8, TAG_CONT_U8) => {}
    //                         _ => err!()
    //                     }
    //                 }
    //                 _ => err!()
    //             }
    //         }
    //     }
    // }

    // // https://tools.ietf.org/html/rfc3629
    // static UTF8_CHAR_WIDTH: [u8; 256] = [
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    // 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
    // 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    // 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
    // 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    // 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
    // 0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
    // 2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
    // 3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
    // 4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
    // ];

    // const CONT_MASK: u8 = 0b0011_1111;

    // const TAG_CONT_U8: u8 = 0b1000_0000;

    // pub unsafe fn from_utf8_unchecked<'a>(v: &'a [u8]) -> &'a str {
    //     mem::transmute(v)
    // }

    #[test]
    fn valid_up_to_test1() {
	let v: &[u8] = &[0xff];
	let from_utf8: Result<&str, Utf8Error> = from_utf8(v);
	let unwrap_err: Utf8Error = from_utf8.unwrap_err();

	let valid_up_to: usize = unwrap_err.valid_up_to();
	assert_eq!(valid_up_to, 0);

	let result: String = format!("{}", unwrap_err);
	assert_eq!(result, "invalid utf-8: invalid byte near index 0");
    }

    #[test]
    fn valid_up_to_test2() {
	let v: &[u8] = &[0x0, 0xff];
	let from_utf8: Result<&str, Utf8Error> = from_utf8(v);
	let unwrap_err: Utf8Error = from_utf8.unwrap_err();

	let valid_up_to: usize = unwrap_err.valid_up_to();
	assert_eq!(valid_up_to, 1);

	let result: String = format!("{}", unwrap_err);
	assert_eq!(result, "invalid utf-8: invalid byte near index 1");
    }
}
