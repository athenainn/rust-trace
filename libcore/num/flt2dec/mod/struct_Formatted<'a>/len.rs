#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::Formatted;
    use core::num::flt2dec::Part::{Zero, Num, Copy};

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

    // #[derive(Clone)]
    // pub struct Formatted<'a> {
    //     /// A byte slice representing a sign, either `""`, `"-"` or `"+"`.
    //     pub sign: &'static [u8],
    //     /// Formatted parts to be rendered after a sign and optional zero padding.
    //     pub parts: &'a [Part<'a>],
    // }

    // impl<'a> Formatted<'a> {
    //     /// Returns the exact byte length of combined formatted result.
    //     pub fn len(&self) -> usize {
    //         let mut len = self.sign.len();
    //         for part in self.parts {
    //             len += part.len();
    //         }
    //         len
    //     }
    //
    //     /// Writes all formatted parts into the supplied buffer.
    //     /// Returns the number of written bytes, or `None` if the buffer is not enough.
    //     /// (It may still leave partially written bytes in the buffer; do not rely on that.)
    //     pub fn write(&self, out: &mut [u8]) -> Option<usize> {
    //         if out.len() < self.sign.len() { return None; }
    //         bytes::copy_memory(self.sign, out);
    //
    //         let mut written = self.sign.len();
    //         for part in self.parts {
    //             match part.write(&mut out[written..]) {
    //                 Some(len) => { written += len; }
    //                 None => { return None; }
    //             }
    //         }
    //         Some(written)
    //     }
    // }

    #[test]
    fn len_test1() {
	let formatted: Formatted = Formatted {
	    sign: "".as_bytes(),
	    parts: &[
		Num(123), Zero(2), Num(456), Copy("9".as_bytes())
	    ]
	};
	let len: usize = formatted.len();

	assert_eq!(len, 0 + 3 + 2 + 3 + 1);
    }

    #[test]
    fn len_test2() {
	let formatted: Formatted = Formatted {
	    sign: "+".as_bytes(),
	    parts: &[
		Num(123), Zero(2), Num(456), Copy("9".as_bytes())
	    ]
	};
	let len: usize = formatted.len();

	assert_eq!(len, 1 + 3 + 2 + 3 + 1);
    }

    #[test]
    fn len_test3() {
	let formatted: Formatted = Formatted {
	    sign: "-".as_bytes(),
	    parts: &[
		Num(123), Zero(2), Num(456), Copy("9".as_bytes())
	    ]
	};
	let len: usize = formatted.len();

	assert_eq!(len, 1 + 3 + 2 + 3 + 1);
    }
}
