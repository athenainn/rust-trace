#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    type T = u32;
    type U = &'static str;

    #[test]
    fn and_test1() {
	let x: Option<T> = Some(2);
	let optb: Option<U> = None;
	let z: Option<U> = x.and::<U>(optb);

	assert_eq!(z, None);
    }

    #[test]
    fn and_test2() {
	let x: Option<T> = None;
	let optb: Option<U> = Some("foo");
	let z: Option<U> = x.and(optb);

	assert_eq!(z, None);
    }

    #[test]
    fn and_test3() {
	let x: Option<T> = Some(2);
	let optb: Option<U> = Some("foo");
	let z: Option<U> = x.and(optb);

	assert_eq!(z, Some("foo"));
    }
}
