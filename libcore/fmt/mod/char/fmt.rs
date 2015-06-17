#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl Debug for char {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         use char::CharExt;
    //         try!(write!(f, "'"));
    //         for c in self.escape_default() {
    //             try!(write!(f, "{}", c));
    //         }
    //         write!(f, "'")
    //     }
    // }

    // impl Display for char {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         f.write_char(*self)
    //     }
    // }

    #[test]
    fn fmt_test1() {
	let value: char = '\u{1f37a}';
	let output: String = format!("{:?}", value);

	assert_eq!(output, "\'\\u{1f37a}\'".to_string());
    }

    #[test]
    fn fmt_test2() {
	let value: char = '\u{1f37a}';
	let output: String = format!("{}", value);

	assert_eq!(output, "\u{1f37a}".to_string());
    }
}
