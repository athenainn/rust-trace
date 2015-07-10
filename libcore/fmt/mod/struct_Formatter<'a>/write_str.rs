#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::Formatter;
    use core::fmt::Display;
    use core::fmt::write;
    use core::fmt::Result;

    struct A<T> {
	value: T
    }

    // pub struct Formatter<'a> {
    //     flags: u32,
    //     fill: char,
    //     align: rt::v1::Alignment,
    //     width: Option<usize>,
    //     precision: Option<usize>,
    //
    //     buf: &'a mut (Write+'a),
    //     curarg: slice::Iter<'a, ArgumentV1<'a>>,
    //     args: &'a [ArgumentV1<'a>],
    // }

    // impl<'a> Write for Formatter<'a> {
    //     fn write_str(&mut self, s: &str) -> Result {
    //         self.buf.write_str(s)
    //     }
    //
    //     fn write_char(&mut self, c: char) -> Result {
    //         self.buf.write_char(c)
    //     }
    //
    //     fn write_fmt(&mut self, args: Arguments) -> Result {
    //         write(self.buf, args)
    //     }
    // }

    // pub trait Display {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl Display for A<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    try!(fmt.write_str("A { value: ["));
	    try!(write(fmt, format_args!("{}", self.value)));
	    fmt.write_str("] }")
	}
    }

    type T = u32;

    #[test]
    fn write_str_test1() {
	let a: A<T> = A { value: 68 };
	let output: String = format!("{}", a);

	assert_eq!(output, "A { value: [68] }".to_string());
    }
}
