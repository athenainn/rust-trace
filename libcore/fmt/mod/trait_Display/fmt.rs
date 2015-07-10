#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::Display;
    use core::fmt::Formatter;
    use core::fmt::write;
    use core::fmt::Result;

    struct A<T> {
	value: T
    }

    // pub trait Display {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl Display for A<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    write(fmt, format_args!("A {{ value: [{}] }}", self.value))
	}
    }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let a: A<T> = A { value: 68 };
	let output: String = format!("{}", a);

	assert_eq!(output, "A { value: [68] }".to_string());
    }
}
