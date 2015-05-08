#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[derive(Debug)]
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
    fn cloned_test() {
	let mut a: A = A { value: 68 };
	let x: Option<&A> = Some::<&A>(&a);
	assert_eq!(x.unwrap().value, a.value);

	let mut y: Option<A> = x.cloned();
	assert_eq!(y.unwrap().value, a.value);
    }
}
