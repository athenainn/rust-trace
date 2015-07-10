#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::RangeFrom;

    // pub struct RangeFrom<Idx> {
    //     /// The lower bound of the range (inclusive).
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub start: Idx,
    // }

    // impl<Idx: fmt::Debug> fmt::Debug for RangeFrom<Idx> {
    //     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //         write!(fmt, "{:?}..", self.start)
    //     }
    // }

    type T = i32;

    #[test]
    fn fmt_test1() {
	let rangefrom: RangeFrom<T> = RangeFrom { start: 100 };
	let message: String = format!("{:?}", rangefrom);

	assert_eq!(message, "100..".to_string());
    }

    #[test]
    fn fmt_test2() {
	let rangefrom: RangeFrom<T> = 100..;
	let message: String = format!("{:?}", rangefrom);

	assert_eq!(message, "100..".to_string());
    }
}
