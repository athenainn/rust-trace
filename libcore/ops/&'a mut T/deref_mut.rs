#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::DerefMut;

    struct A<T> {
	value: T
    }

    type T = i32;

    #[test]
    fn deref_mut_test1() {
	let mut a: A<T> = A { value: 68 };

	{
	    let a_ptr: &mut A<T> = &mut a;
	    (*a_ptr).value = 500;
	}

	assert_eq!(a.value, 500);
    }

    #[test]
    fn deref_mut_test2() {
	let mut a: A<T> = A { value: 68 };

	{
	    let mut a_ptr: &mut A<T> = &mut a;
	    a_ptr.deref_mut().value = 500;
	}

	assert_eq!(a.value, 500);
    }

    #[test]
    fn deref_mut_test3() {
	let mut a: A<T> = A { value: 68 };

	{
	    let mut a_ptr: &mut A<T> = &mut a;
	    (*a_ptr.deref_mut()).value = 500;
	}

	assert_eq!(a.value, 500);
    }
}
