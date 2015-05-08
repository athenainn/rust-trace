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
	let mut x: A = A { value: 68 };
	assert_eq!(x.value, 68);

	x.value = 500;
	let y: A = x.clone();
	assert_eq!(y.value, 500);
    }
}
