#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::clone::Clone;

    // #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Option<T> {
    //     /// No value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     None,
    //     /// Some value `T`
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Some(T)
    // }

    // impl<'a, T: Clone> Option<&'a T> {
    //     /// Maps an Option<&T> to an Option<T> by cloning the contents of the Option.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn cloned(self) -> Option<T> {
    //         self.map(|t| t.clone())
    //     }
    // }

    struct A<T> {
	value: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self
	{
	    A { value: self.value  }
	}
    }

    type T = i32; // T: Clone

    #[test]
    fn cloned_test1() {
	let a: A<T> = A { value: 68 };
	let x: Option<&A<T>> = Some::<&A<T>>(&a);
	let cloned: Option<A<T>> = x.cloned();

	assert_eq!(x.unwrap().value, 68);
	assert_eq!(cloned.unwrap().value, 68);
    }
}
