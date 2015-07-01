#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::Part::{self, Zero, Num, Copy};

    // #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    // pub enum Part<'a> {
    //     /// Given number of zero digits.
    //     Zero(usize),
    //     /// A literal number up to 5 digits.
    //     Num(u16),
    //     //     /// A verbatim copy of given bytes.
    //     Copy(&'a [u8]),
    // }

    // impl<'a> Part<'a> {
    //     /// Returns the exact byte length of given part.
    //     pub fn len(&self) -> usize {
    //         match *self {
    //             Part::Zero(nzeroes) => nzeroes,
    //             Part::Num(v) => if v < 1_000 { if v < 10 { 1 } else if v < 100 { 2 } else { 3 } }
    //                             else { if v < 10_000 { 4 } else { 5 } },
    //             Part::Copy(buf) => buf.len(),
    //         }
    //     }
    //
    //     /// Writes a part into the supplied buffer.
    //     /// Returns the number of written bytes, or `None` if the buffer is not enough.
    //     /// (It may still leave partially written bytes in the buffer; do not rely on that.)
    //     pub fn write(&self, out: &mut [u8]) -> Option<usize> {
    //         let len = self.len();
    //         if out.len() >= len {
    //             match *self {
    //                 Part::Zero(nzeroes) => {
    //                     for c in &mut out[..nzeroes] { *c = b'0'; }
    //                 }
    //                 Part::Num(mut v) => {
    //                     for c in out[..len].iter_mut().rev() {
    //                         *c = b'0' + (v % 10) as u8;
    //                         v /= 10;
    //                     }
    //                 }
    //                 Part::Copy(buf) => {
    //                     bytes::copy_memory(buf, out);
    //                 }
    //             }
    //             Some(len)
    //         } else {
    //             None
    //         }
    //     }
    // }

    #[test]
    fn write_test1() {
	let nzeroes: usize = 6;
	let part: Part = Zero(nzeroes);
	let out: &mut [u8] = &mut [
	    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
	];
	let result: Option<usize> = part.write(out);

	assert_eq!(result, Some::<usize>(6));
	assert_eq!(out, &mut [ b'0', b'0', b'0', b'0', b'0', b'0', 0x00, 0x00 ]);
    }

    #[test]
    fn write_test2() {
	let v: u16 = 6;
	let part: Part = Num(v);
	let out: &mut [u8] = &mut [
	    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
	];
	let result: Option<usize> = part.write(out);

	assert_eq!(result, Some::<usize>(1));
	assert_eq!(out, &mut [ b'6', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ]);
    }

    #[test]
    fn write_test3() {
	let v: u16 = 68;
	let part: Part = Num(v);
	let out: &mut [u8] = &mut [
	    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
	];
	let result: Option<usize> = part.write(out);

	assert_eq!(result, Some::<usize>(2));
	assert_eq!(out, &mut [ b'6', b'8', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ]);
    }

    #[test]
    fn write_test4() {
	let v: u16 = 500;
	let part: Part = Num(v);
	let out: &mut [u8] = &mut [
	    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
	];
	let result: Option<usize> = part.write(out);

	assert_eq!(result, Some::<usize>(3));
	assert_eq!(out, &mut [ b'5', b'0', b'0', 0x00, 0x00, 0x00, 0x00, 0x00 ]);
    }

    #[test]
    fn write_test5() {
	let v: u16 = 1688;
	let part: Part = Num(v);
	let out: &mut [u8] = &mut [
	    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
	];
	let result: Option<usize> = part.write(out);

	assert_eq!(result, Some::<usize>(4));
	assert_eq!(out, &mut [ b'1', b'6', b'8', b'8', 0x00, 0x00, 0x00, 0x00 ]);
    }

    #[test]
    fn write_test6() {
	let v: u16 = 13579;
	let part: Part = Num(v);
	let out: &mut [u8] = &mut [
	    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
	];
	let result: Option<usize> = part.write(out);

	assert_eq!(result, Some::<usize>(5));
	assert_eq!(out, &mut [ b'1', b'3', b'5', b'7', b'9', 0x00, 0x00, 0x00 ]);
    }

    #[test]
    fn write_test7() {
	let bytes: &[u8]= "299792458".as_bytes();
	let part: Part = Copy(bytes);
	let out: &mut [u8] = &mut [
	    0x00, 0x00, 0x00, 0x00, 0x00,
	    0x00, 0x00, 0x00, 0x00, 0x00
	];
	let result: Option<usize> = part.write(out);

	assert_eq!(result, Some::<usize>(9));
	assert_eq!(out, &mut [
	    b'2', b'9', b'9', b'7', b'9',
	    b'2', b'4', b'5', b'8', 0x00
	]);
    }

    #[test]
    fn write_test8() {
	let out: &mut [u8] = &mut [ 0x00 ];

	let nzeroes: usize = 68;
	let part: Part = Zero(nzeroes);
	let result: Option<usize> = part.write(out);
	assert_eq!(result, None::<usize>);

	let v: u16 = 68;
	let part: Part = Num(v);
	let result: Option<usize> = part.write(out);
	assert_eq!(result, None::<usize>);

	let bytes: &[u8]= "299792458".as_bytes();
	let part: Part = Copy(bytes);
	let result: Option<usize> = part.write(out);
	assert_eq!(result, None::<usize>);
    }
}
