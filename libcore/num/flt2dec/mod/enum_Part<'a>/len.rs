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
    fn len_test1() {
	let nzeroes: usize = 68;
	let part: Part = Zero(nzeroes);
	let len: usize = part.len();

	assert_eq!(len, nzeroes);
    }

    #[test]
    fn len_test2() {
	for v in 0..10u16 {
	    let part: Part = Num(v);
	    let len: usize = part.len();

	    assert_eq!(len, 1);
	}
    }

    #[test]
    fn len_test3() {
	for v in 10..100u16 {
	    let part: Part = Num(v);
	    let len: usize = part.len();

	    assert_eq!(len, 2);
	}
    }

    #[test]
    fn len_test4() {
	for v in 100..1000u16 {
	    let part: Part = Num(v);
	    let len: usize = part.len();

	    assert_eq!(len, 3);
	}
    }

    #[test]
    fn len_test5() {
	for v in 1000..10000u16 {
	    let part: Part = Num(v);
	    let len: usize = part.len();

	    assert_eq!(len, 4);
	}
    }

    #[test]
    #[allow(overflowing_literals)]
    fn len_test6() {
	for v in 10000..0x10000u16 {
	    let part: Part = Num(v);
	    let len: usize = part.len();

	    assert_eq!(len, 5);
	}
    }

    #[test]
    fn len_test7() {
	let bytes: &[u8]= "299792458".as_bytes();
	let part: Part = Copy(bytes);
	let len: usize = part.len();

	assert_eq!(len, 9);
    }
}
