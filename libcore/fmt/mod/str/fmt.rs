#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl Debug for str {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         try!(write!(f, "\""));
    //         for c in self.chars().flat_map(|c| c.escape_default()) {
    //             try!(write!(f, "{}", c));
    //         }
    //         write!(f, "\"")
    //     }
    // }

    // impl Display for str {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         f.pad(self)
    //     }
    // }

    #[test]
    fn fmt_test1() {
	let value: &'static str = "\t精\t誠\n";
	let output: String = format!("{:?}", value);

	assert_eq!(output, "\"\\t\\u{7cbe}\\t\\u{8aa0}\\n\"".to_string());
    }

    #[test]
    fn fmt_test2() {
	let value: &'static str = "\t精\t誠\n";
	let output: String = format!("{}", value);

	assert_eq!(output, "\t\u{7cbe}\t\u{8aa0}\n".to_string());
    }
}
