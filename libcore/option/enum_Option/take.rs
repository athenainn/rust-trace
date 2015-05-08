#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    type T = u32;

    #[test]
    fn take_test1() {
	let mut x: Option<T> = Some::<T>(2);
	let y: Option<T> = x.take();

	assert_eq!(x, None::<T>);
	assert_eq!(y, Some::<T>(2));
    }

    #[test]
    fn take_test2() {
	let mut x: Option<T> = None::<T>;
	let y: Option<T> = x.take();

	assert_eq!(x, None::<T>);
	assert_eq!(y, None::<T>);
    }
}
