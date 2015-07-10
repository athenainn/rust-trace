#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::LowerHex;
    use core::fmt::Formatter;
    use core::fmt::write;
    use core::fmt::Result;

    struct A<T> {
	value: T
    }

    // pub trait LowerHex {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl LowerHex for A<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    write(fmt, format_args!("A {{ value: [0x{:x}] }}", self.value))
	}
    }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let a: A<T> = A { value: 500 };
	let output: String = format!("{:x}", a);

	assert_eq!(output, "A { value: [0x1f4] }".to_string());
    }
}
