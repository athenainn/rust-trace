#![feature(core, collections, collections_range)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::range::RangeArgument;

    use core::ops::RangeFull;

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

    // impl<T> RangeArgument<T> for RangeFull {}

    type T = usize;

    #[test]
    fn end_test1() {
	let rangefull: RangeFull = RangeFull;
	let start: Option<&T> = rangefull.start();
	let end: Option<&T> = rangefull.end();

	assert_eq!(start, None::<&T>);
	assert_eq!(end, None::<&T>);
    }

    #[test]
    fn end_test2() {
	let rangefull: RangeFull = ..;
	let start: Option<&T> = rangefull.start();
	let end: Option<&T> = rangefull.end();

	assert_eq!(start, None::<&T>);
	assert_eq!(end, None::<&T>);
    }
}
