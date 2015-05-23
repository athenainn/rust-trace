#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Drop;

    struct HasDrop;

    impl Drop for HasDrop {
	fn drop(&mut self) {
	    assert!(false);
	}
    }

    #[test]
    #[should_panic]
    fn drop_test1() {
	let x: HasDrop = HasDrop;
    }
}
