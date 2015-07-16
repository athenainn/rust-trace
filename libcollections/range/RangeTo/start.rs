#![feature(core, collections, collections_range)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::range::RangeArgument;

    use core::ops::RangeTo;

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

    // impl<T> RangeArgument<T> for RangeTo<T> {
    //     fn end(&self) -> Option<&T> { Some(&self.end) }
    // }

    type T = usize;

    #[test]
    fn start_test1() {
	let rangeto: RangeTo<T> = RangeTo { end: 500 };
	let start: Option<&T> = rangeto.start();
	let end: Option<&T> = rangeto.end();

	assert_eq!(start, None::<&T>);
	assert_eq!(end, Some::<&T>(&500));
    }

    #[test]
    fn start_test2() {
	let rangeto: RangeTo<T> = ..500;
	let start: Option<&T> = rangeto.start();
	let end: Option<&T> = rangeto.end();

	assert_eq!(start, None::<&T>);
	assert_eq!(end, Some::<&T>(&500));
    }
}
