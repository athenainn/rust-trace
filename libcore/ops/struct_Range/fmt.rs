#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Range;

    // pub struct Range<Idx> {
    //     /// The lower bound of the range (inclusive).
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub start: Idx,
    //     /// The upper bound of the range (exclusive).
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub end: Idx,
    // }

    // impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    //     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //         write!(fmt, "{:?}..{:?}", self.start, self.end)
    //     }
    // }

    type T = i32;

    #[test]
    fn fmt_test1() {
	let range: Range<usize> = Range { start: 100, end: 200 };
	let message: String = format!("{:?}", range);

	assert_eq!(message, "100..200".to_string());
    }

    #[test]
    fn fmt_test2() {
	let range: Range<usize> = 100..200;
	let message: String = format!("{:?}", range);

	assert_eq!(message, "100..200".to_string());
    }
}
