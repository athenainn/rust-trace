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
    }

    #[test]
    fn clone_test1() {
	let x: A = A { value: 68 };
	assert_eq!(x.value, 68);

	let mut y: A = x.clone();
	y.value = 500;
	assert_eq!(x.value, 68);
	assert_eq!(y.value, 500);
    }
}
