#![feature(core, collections, collections_range)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::range::RangeArgument;

    use core::ops::Range;

    // pub trait RangeArgument<T> {
    //     /// Start index (inclusive)
    //     ///
    //     /// Return start value if present, else `None`.
    //     fn start(&self) -> Option<&T> { None }
    //
    //     /// End index (exclusive)
    //     ///
    //     /// Return end value if present, else `None`.
    //     fn end(&self) -> Option<&T> { None }
    // }

    // impl<T> RangeArgument<T> for Range<T> {
    //     fn start(&self) -> Option<&T> { Some(&self.start) }
    //     fn end(&self) -> Option<&T> { Some(&self.end) }
    // }

    type T = usize;

    #[test]
    fn end_test1() {
	let range: Range<T> = Range { start: 68, end: 500 };
	let start: Option<&T> = range.start();
	let end: Option<&T> = range.end();

	assert_eq!(start, Some::<&T>(&68));
	assert_eq!(end, Some::<&T>(&500));
    }

    #[test]
    fn end_test2() {
	let range: Range<T> = 68..500;
	let start: Option<&T> = range.start();
	let end: Option<&T> = range.end();

	assert_eq!(start, Some::<&T>(&68));
	assert_eq!(end, Some::<&T>(&500));
    }
}
