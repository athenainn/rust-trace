#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IntoIter;

    #[test]
    fn into_iter_test1() {
	type T = &'static str;
	let x: Option<T> = Some::<T>("string");
	let i: IntoIter<T> = x.into_iter();
	let v: Vec<T> = i.collect();

	assert_eq!(v, ["string"]);
	assert_eq!(v, &["string"]);
    }
    #[test]
    fn into_iter_test2() {
	type T = &'static str;
	let x: Option<T> = None::<T>;
	let i: IntoIter<T> = x.into_iter();
	let v: Vec<T> = i.collect();

	assert!(v.is_empty());
    }
    #[test]
    fn into_iter_test3() {
	type T = &'static str;
	let x: Option<T> = Some::<T>("string");
	let mut i: IntoIter<T> = x.into_iter();

	assert_eq!(i.next(), Some::<T>("string"));
    }
}
