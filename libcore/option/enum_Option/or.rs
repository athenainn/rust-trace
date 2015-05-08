#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    type T = u32;

    #[test]
    fn or_test1() {
	let x: Option<T> = Some(2);
	let optb: Option<T> = None;
	let z: Option<T> = x.or(optb);

	assert_eq!(z, Some::<T>(2));
    }

    #[test]
    fn or_test2() {
	let x: Option<T> = None;
	let optb: Option<T> = Some(100);
	let z: Option<T> = x.or(optb);

	assert_eq!(z, Some::<T>(100));
    }

    #[test]
    fn or_test3() {
	let x: Option<T> = Some(2);
	let optb: Option<T> = Some(100);
	let z: Option<T> = x.or(optb);

	assert_eq!(z, Some::<T>(2));
    }

    #[test]
    fn or_test4() {
	let x: Option<T> = None;
	let optb: Option<T> = None;
	let z: Option<T> = x.or(optb);

	assert_eq!(z, None::<T>);
    }
}
