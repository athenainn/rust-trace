#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::fmt::Result;
    use core::fmt::Error;
    use core::fmt::Write;
    use core::fmt::ArgumentV1;
    use core::fmt::Arguments;
    use core::fmt::Display;

    use collections::vec::Vec;

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

    #[test]
    fn write_fmt_test1() {
	let mut a: A = A::new();
	let arg1: &'static str = "Hello";
	let arg2: &'static str = "World";
	let result: Result = a.write_fmt(format_args!("{}, {}!", arg1, arg2));

	match result {
	    Ok(()) => assert!(true),
	    Err(Error) => assert!(false)
	}

	assert_eq!(a.buf, "Hello, World!".as_bytes());
    }

    #[test]
    fn write_fmt_test2() {
	let mut a: A = A::new();

	let pieces: &[&'static str]  = &["", ", ", "!"];

	let arg1: &'static str = "Hello";
	let arg2: &'static str = "World";
	let args: &[ArgumentV1] = &[
	    ArgumentV1::new(&arg1, Display::fmt),
	    ArgumentV1::new(&arg2, Display::fmt)
	];
	let args: Arguments = Arguments::new_v1(pieces, args);

	let result: Result = a.write_fmt(args);

	match result {
	    Ok(()) => assert!(true),
	    Err(Error) => assert!(false)
	}

	assert_eq!(a.buf, "Hello, World!".as_bytes());
    }
}
