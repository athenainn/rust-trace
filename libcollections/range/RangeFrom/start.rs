#![feature(core, collections, collections_range)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::range::RangeArgument;

    use core::ops::RangeFrom;

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

    // impl<T> RangeArgument<T> for RangeFrom<T> {
    //     fn start(&self) -> Option<&T> { Some(&self.start) }
    // }

    type T = usize;

    #[test]
    fn start_test1() {
	let rangefrom: RangeFrom<T> = RangeFrom { start: 68 };
	let start: Option<&T> = rangefrom.start();
	let end: Option<&T> = rangefrom.end();

	assert_eq!(start, Some::<&T>(&68));
	assert_eq!(end, None::<&T>);
    }

    #[test]
    fn start_test2() {
	let rangefrom: RangeFrom<T> = 68..;
	let start: Option<&T> = rangefrom.start();
	let end: Option<&T> = rangefrom.end();

	assert_eq!(start, Some::<&T>(&68));
	assert_eq!(end, None::<&T>);
    }
}
