#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::RangeFull;

    // pub struct RangeFull;

    // impl fmt::Debug for RangeFull {
    //     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //         write!(fmt, "..")
    //     }
    // }

    type T = i32;

    #[test]
    fn fmt_test1() {
	let rangefull: RangeFull = RangeFull;
	let message: String = format!("{:?}", rangefull);

	assert_eq!(message, "..".to_string());
    }

    #[test]
    fn fmt_test2() {
	let rangefull: RangeFull = ..;
	let message: String = format!("{:?}", rangefull);

	assert_eq!(message, "..".to_string());
    }
}
