#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    struct A {
	value: u32
    }

    impl Clone for A {
	fn clone(&self) -> Self
	{
	    A { value: self.value  }
	}

	// fn clone_from(&mut self, source: &Self) {
	//     *self = source.clone()
	// }
    }

    #[test]
    fn clone_from_test1() {
	let mut x: A = A { value: 68 };
	assert_eq!(x.value, 68);

	let mut y: A = A { value: 0 };
	assert_eq!(y.value, 0);

	x.value = 500;
	y.clone_from(&x);
	assert_eq!(y.value, 500);
    }
}
