#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::marker::Reflect;
    use core::any::Any;

    #[test]
    fn reflect_test1() {
	fn foo<T:Reflect + 'static>(x: &T) {
	    let any: &Any = x;
	    if any.is::<u32>() { assert!(true); }
	    else { assert!(false); }
	}
	foo(&68u32);
    }
}
