#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::Result;
    use core::fmt::Error;

    use core::any::Any;

    // pub type Result = result::Result<(), Error>;

    // #[derive(Copy, Clone, Debug)]
    // pub struct Error;

    #[test]
    fn result_test1() {
	let x: Result = Ok::<(), Error>(());
	let unwrap: () = x.unwrap();
	let result: bool = (&unwrap as &Any).is::<()>();

	assert_eq!(result, true);
    }

    #[test]
    fn result_test2() {
	let x: Result = Err::<(), Error>(Error);
	let unwrap_err: Error = x.unwrap_err();
	let result: bool = (&unwrap_err as &Any).is::<Error>();

	assert_eq!(result, true);
    }
}
