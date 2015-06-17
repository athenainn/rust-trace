#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::Pointer;
    use core::fmt::Formatter;
    use core::fmt::write;
    use core::fmt::Result;

    struct A<T> {
	value: *const T
    }

    struct B<T> {
	value: *mut T
    }

    // pub trait Pointer {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl Pointer for A<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    write(fmt, format_args!("A {{ value: [{:p}] }}", self.value))
	}
    }

    impl Pointer for B<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    write(fmt, format_args!("B {{ value: [{:p}] }}", self.value))
	}
    }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: T = 68;
	let a: A<T> = A { value: &value };
	let _: String = format!("{:p}", a); // `"A { value: [0x101e00c7c] }"`
    }

    #[test]
    fn fmt_test2() {
	let mut value: T = 68;
	let b: B<T> = B { value: &mut value };
	let _: String = format!("{:p}", b); // `"B { value: [0x105606c7c] }"`
    }
}
