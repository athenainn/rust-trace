#![feature(core, collections)]

extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::iter::FromIterator;

    use collections::vec;

    struct A<T> {
	data: Vec<T>
    }

    impl<T> FromIterator<T> for A<T> {
	fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
	    let mut v = vec!();
	    v.extend(iter);
	    A { data: v }
	}
    }

    type T = i32;

    #[test]
    fn from_iter_test1() {
	let v: vec::Vec<T> = vec!(1, 2, 3, 4, 5);
	let a: A<T> = A::<T>::from_iter(v.clone());

	assert_eq!(a.data, v);
    }

    #[test]
    fn from_iter_test2() {
	let v: vec::Vec<T> = vec!(1, 2, 3, 4, 5);
	let a: A<T> = FromIterator::from_iter(v.clone());

	assert_eq!(a.data, v);
    }
}
