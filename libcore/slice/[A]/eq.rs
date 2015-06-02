#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::PartialEq;

    struct A<T> {
	value: T
    }

    struct B<T> {
	value: T
    }

    impl PartialEq<B<T>> for A<T> {
	fn eq(&self, other: &B<T>) -> bool {
	    self.value == other.value
	}

	// fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
    }

    // impl<A, B> PartialEq<[B]> for [A] where A: PartialEq<B> {
    //     fn eq(&self, other: &[B]) -> bool {
    //         self.len() == other.len() &&
    //             order::eq(self.iter(), other.iter())
    //     }
    //     fn ne(&self, other: &[B]) -> bool {
    //         self.len() != other.len() ||
    //             order::ne(self.iter(), other.iter())
    //     }
    // }

    type T = i32; // T: PartialEq
    type AA = A<T>;
    type BB = B<T>;

    #[test]
    fn eq_test1() {
	let slice: &[AA] = &[
		AA { value: 68 },
		AA { value: 500 },
		AA { value: 999 }
	    ];
	let other: &[BB] = &[
		BB { value: 68 },
		BB { value: 500 },
		BB { value: 999 }
	    ];
	let eq: bool = slice.eq(other);

	assert_eq!(eq, true);
    }

    #[test]
    fn eq_test2() {
	let slice: &[AA] = &[
		AA { value: 68 },
		AA { value: 500 },
		AA { value: 999 }
	    ];
	let other: &[BB] = &[
		BB { value: 68 },
		BB { value: 500 },
		BB { value: 999 }
	    ];
	let eq: bool = slice == other;

	assert_eq!(eq, true);
    }

    #[test]
    fn eq_test3() {
	let slice: &[AA] = &[
		AA { value: 68 },
		AA { value: 500 },
		AA { value: 999 }
	    ];
	let other: &[BB] = &[
		BB { value: 168 },
		BB { value: 500 },
		BB { value: 999 }
	    ];
	let eq: bool = slice.eq(other);

	assert_eq!(eq, false);
    }


    #[test]
    fn eq_test4() {
	let slice: &[AA] = &[
		AA { value: 68 },
		AA { value: 500 },
		AA { value: 999 }
	    ];
	let other: &[BB] = &[
		BB { value: 168 },
		BB { value: 500 },
		BB { value: 999 }
	    ];
	let eq: bool = slice == other;

	assert_eq!(eq, false);
    }
}
