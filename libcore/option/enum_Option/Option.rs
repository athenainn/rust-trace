#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
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

    type T = u32;

    #[test]
    fn option_test1() {
	let x: Option<T> = Some::<T>(2);

	match x {
	    Some(z) => { assert_eq!(z, 2); }
	    None => { assert!(false); }
	}
    }

    #[test]
    fn option_test2() {
	let x: Option<T> = None::<T>;

	match x {
	    Some(_) => { assert!(false); }
	    None => { assert!(true); }
	}
    }
}
