#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    // #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Result<T, E> {
    //     /// Contains the success value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Ok(T),
    //
    //     /// Contains the error value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Err(E)
    // }

    type T = i32;
    type E = &'static str;

    #[test]
    fn result_test1() {
	let x: Result<T, E> = Ok::<T, E>(-3);
	let y: Result<T, E> = Err::<T, E>("Some error message");

	match x {
	    Ok(z) => { assert_eq!(z, -3); }
	    _ => { assert!(false); }
	}

	match y {
	    Err(m) => { assert_eq!(m, "Some error message"); }
	    _ => { assert!(false); }
	}
    }
}
