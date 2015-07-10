#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! try {
    //     ($e:expr) => ({
    //         use $crate::result::Result::{Ok, Err};
    //
    //         match $e {
    //             Ok(e) => e,
    //             Err(e) => return Err(e),
    //         }
    //     })
    // }

    type T = u32;
    type E = &'static str;

    fn ok() -> Result<T, E> { Ok::<T, E>(68) }
    fn err() -> Result<T, E> { Err::<T, E>("Error") }

    #[test]
    fn try_test1() {
	fn foo() -> Result<T, E> {
	    let x: T = try!(ok());
	    Ok::<T, E>(x)
	}

	let result: Result<T, E> = foo();

	assert_eq!(result, Ok::<T, E>(68));
    }

    #[test]
    fn try_test2() {
	fn foo() -> Result<T, E> {
	    let x: T = try!(err());
	    Ok::<T, E>(x)
	}

	let result: Result<T, E> = foo();

	assert_eq!(result, Err::<T, E>("Error"));
    }
}
