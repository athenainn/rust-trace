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

    // impl<T, U> Into<U> for T where U: From<T> {
    //     fn into(self) -> U {
    //         U::from(self)
    //     }
    // }

    type T = i32;

    #[test]
    fn into_test1() {
	let value: T = 68;
	let a: A<T> = A { value: value };
	let into: A<T> = a.into();

	assert_eq!(into.value, value);
    }
}
