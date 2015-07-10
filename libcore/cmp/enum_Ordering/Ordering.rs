#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    type T = u32;

    #[test]
    fn ordering_test1() {
	let x: Ordering = Less;

	assert_eq!(x as usize, -1);
    }

    #[test]
    fn ordering_test2() {
	let x: Ordering = Equal;

	assert_eq!(x as usize, 0);
    }

    #[test]
    fn ordering_test3() {
	let x: Ordering = Greater;

	assert_eq!(x as usize, 1);
    }
}
