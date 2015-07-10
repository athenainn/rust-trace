#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::RangeTo;

    // pub struct RangeTo<Idx> {
    //     /// The upper bound of the range (exclusive).
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub end: Idx,
    // }


    // impl<Idx: fmt::Debug> fmt::Debug for RangeTo<Idx> {
    //     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //         write!(fmt, "..{:?}", self.end)
    //     }
    // }

    type T = i32;

    #[test]
    fn fmt_test1() {
	let rangeto: RangeTo<T> = RangeTo { end: 200 };
	let message: String = format!("{:?}", rangeto);

	assert_eq!(message, "..200".to_string());
    }

    #[test]
    fn fmt_test2() {
	let rangeto: RangeTo<T> = ..200;
	let message: String = format!("{:?}", rangeto);

	assert_eq!(message, "..200".to_string());
    }
}
