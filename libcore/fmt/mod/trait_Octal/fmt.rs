#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::Octal;
    use core::fmt::Formatter;
    use core::fmt::write;
    use core::fmt::Result;

    struct A<T> {
	value: T
    }

    // pub trait Octal {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl Octal for A<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    write(fmt, format_args!("A {{ value: [0o{:o}] }}", self.value))
	}
    }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let a: A<T> = A { value: 68 };
	let output: String = format!("{:o}", a);

	assert_eq!(output, "A { value: [0o104] }".to_string());
    }
}
