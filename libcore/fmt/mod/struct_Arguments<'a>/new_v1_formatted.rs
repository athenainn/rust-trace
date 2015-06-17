#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::fmt::write;
    use core::fmt::Result;
    use core::fmt::Error;
    use core::fmt::Write;
    use core::fmt::ArgumentV1;
    use core::fmt::Arguments;
    use core::fmt::Display;

    use collections::vec::Vec;

    use core::fmt::rt::v1::Argument;
    use core::fmt::rt::v1::Position;
    use core::fmt::rt::v1::FormatSpec;
    use core::fmt::rt::v1::Alignment;
    use core::fmt::rt::v1::Count;

    struct A {
	buf: Vec<u8>
    }

    impl A {
	fn new() -> A {
	    A { buf: vec!() }
	}
    }

    impl Write for A {
	fn write_str(&mut self, s: &str) -> Result {
	    for b in s.bytes() {
		self.buf.push(b);
	    }
	    Ok(())
	}

	// fn write_char(&mut self, c: char) -> Result {
	//     let mut utf_8 = [0u8; 4];
	//     let bytes_written = c.encode_utf8(&mut utf_8).unwrap_or(0);
	//     self.write_str(unsafe { mem::transmute(&utf_8[..bytes_written]) })
	// }

	// fn write_fmt(&mut self, args: Arguments) -> Result {
	//     // This Adapter is needed to allow `self` (of type `&mut
	//     // Self`) to be cast to a Write (below) without
	//     // requiring a `Sized` bound.
	//     struct Adapter<'a,T: ?Sized +'a>(&'a mut T);
	//
	//     impl<'a, T: ?Sized> Write for Adapter<'a, T>
	//         where T: Write
	//     {
	//         fn write_str(&mut self, s: &str) -> Result {
	//             self.0.write_str(s)
	//         }
	//
	//         fn write_fmt(&mut self, args: Arguments) -> Result {
	//             self.0.write_fmt(args)
	//         }
	//     }
	//
	//     write(&mut Adapter(self), args)
	// }
    }

    // pub struct Arguments<'a> {
    //     // Format string pieces to print.
    //     pieces: &'a [&'a str],
    //
    //     // Placeholder specs, or `None` if all specs are default (as in "{}{}").
    //     fmt: Option<&'a [rt::v1::Argument]>,
    //
    //     // Dynamic arguments for interpolation, to be interleaved with string
    //     // pieces. (Every argument is preceded by a string piece.)
    //     args: &'a [ArgumentV1<'a>],
    // }

    // impl<'a> Arguments<'a> {
    //     /// When using the format_args!() macro, this function is used to generate the
    //     /// Arguments structure.
    //     #[doc(hidden)] #[inline]
    //     #[unstable(feature = "core", reason = "internal to format_args!")]
    //     pub fn new_v1(pieces: &'a [&'a str],
    //                   args: &'a [ArgumentV1<'a>]) -> Arguments<'a> {
    //         Arguments {
    //             pieces: pieces,
    //             fmt: None,
    //             args: args
    //         }
    //     }
    //
    //     /// This function is used to specify nonstandard formatting parameters.
    //     /// The `pieces` array must be at least as long as `fmt` to construct
    //     /// a valid Arguments structure. Also, any `Count` within `fmt` that is
    //     /// `CountIsParam` or `CountIsNextParam` has to point to an argument
    //     /// created with `argumentusize`. However, failing to do so doesn't cause
    //     /// unsafety, but will ignore invalid .
    //     #[doc(hidden)] #[inline]
    //     #[unstable(feature = "core", reason = "internal to format_args!")]
    //     pub fn new_v1_formatted(pieces: &'a [&'a str],
    //                             args: &'a [ArgumentV1<'a>],
    //                             fmt: &'a [rt::v1::Argument]) -> Arguments<'a> {
    //         Arguments {
    //             pieces: pieces,
    //             fmt: Some(fmt),
    //             args: args
    //         }
    //     }
    // }

    // #[derive(Copy, Clone)]
    // pub struct Argument {
    //     pub position: Position,
    //     pub format: FormatSpec,
    // }

    #[test]
    fn new_v1_test1() {
	let mut a: A = A::new();

	let pieces: &[&'static str]  = &[ "" ];

	let arg1: &'static str = "Hello, World!";
	let argumentv1: ArgumentV1 = ArgumentV1::new(&arg1, Display::fmt);

	let args: &[ArgumentV1] = &[ argumentv1 ];

	let argument: Argument = Argument {
	    position: Position::At(0),
	    format: FormatSpec {
		fill: ' ',
		align: Alignment::Unknown,
		flags: 0,
		precision: Count::NextParam,
		width: Count::NextParam
	    }
	};
	let fmt: &[Argument] = &[ argument ];


	let args: Arguments = Arguments::new_v1_formatted(pieces, args, fmt);

	let result: Result = write(&mut a, args);

	match result {
	    Ok(()) => assert!(true),
	    Err(Error) => assert!(false)
	}

	assert_eq!(a.buf, "Hello, World!". as_bytes());
    }
}
