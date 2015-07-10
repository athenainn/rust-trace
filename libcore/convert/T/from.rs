#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    struct A<T> {
	value: T
    }

    // impl<T> From<T> for T {
    //     fn from(t: T) -> T { t }
    // }

    type T = i32;

    #[test]
    fn into_test1() {
	let value: T = 68;
	let a: A<T> = A { value: value };
	let from: A<T> = A::<T>::from(a);

	assert_eq!(from.value, value);
    }
}
