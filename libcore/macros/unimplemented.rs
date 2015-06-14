#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! unimplemented {
    //     () => (panic!("not yet implemented"))
    // }

    #[test]
    #[should_panic]
    fn unimplemented_test1() {
	unimplemented!(); // panicked at 'not yet implemented'
    }
}
