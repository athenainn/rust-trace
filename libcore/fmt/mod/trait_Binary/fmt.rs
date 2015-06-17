#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::Binary;
    use core::fmt::Formatter;
    use core::fmt::write;
    use core::fmt::Result;

    struct A<T> {
	value: T
    }

    // pub trait Binary {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl Binary for A<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    write(fmt, format_args!("A {{ value: [{:b}] }}", self.value))
	}
    }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let a: A<T> = A { value: 68 };
	let output: String = format!("{:b}", a);

	assert_eq!(output, "A { value: [1000100] }".to_string());
    }
}
