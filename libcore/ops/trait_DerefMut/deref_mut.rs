#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Deref;
    use core::ops::DerefMut;

    struct A<T> {
	value: T
    }

    impl Deref for A<T> {
	type Target = T;

	fn deref<'a>(&'a self) -> &'a Self::Target {
	    &self.value
	}
    }

    impl DerefMut for A<T> {
	fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
	    &mut self.value
	}
    }


    type T = i32;

    #[test]
    fn deref_mut_test1() {
	let mut a: A<T> = A { value: 68 };

	*a = 500;

	assert_eq!(a.value, 500);
    }

    #[test]
    fn deref_mut_test2() {
	let mut a: A<T> = A { value: 68 };

	*a.deref_mut() = 500;

	assert_eq!(a.value, 500);
    }
}
