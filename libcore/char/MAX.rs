#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::char::MAX;

    // pub const MAX: char = '\u{10ffff}';

    #[test]
    #[allow(non_snake_case)]
    fn MAX_test1() {
	assert_eq!(MAX, '\u{10ffff}');
    }
}
