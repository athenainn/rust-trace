#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::assume;

    // pub fn assume(b: bool);

    #[test]
    fn assume_test1() {
	unsafe {
	    if true {
		// Informs the optimizer that a condition is always true.
		// If the condition is false, the behavior is undefined.
		assume(true);
	    }
	}
    }

    #[test]
    fn assume_test2() {
	unsafe {
	    if false {
	    } else {
		// Informs the optimizer that a condition is always true.
		// If the condition is false, the behavior is undefined.
		assume(true);
	    }
	}
    }
}
