#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::LowerExp;
    use core::fmt::Formatter;
    use core::fmt::write;
    use core::fmt::Result;

    struct A<T> {
	value: T
    }

    // pub trait LowerExp {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl LowerExp for A<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    write(fmt, format_args!("A {{ value: [{:e}] }}", self.value))
	}
    }

    type T = f32;

    #[test]
    fn fmt_test1() {
	let a: A<T> = A { value: 1.68 };
	let output: String = format!("{:e}", a);

	assert_eq!(output, "A { value: [1.68e0] }".to_string());
    }
}
