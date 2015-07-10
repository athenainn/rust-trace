#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     pub unsafe fn as_ref<'a>(&self) -> Option<&'a T> where T: Sized {
    //         if self.is_null() {
    //             None
    //         } else {
    //             Some(&**self)
    //         }
    //     }

    type T = i32;

    #[test]
    fn as_ref_test1() {
	let ptr: *const T = 0 as *const T;
	let result: Option<&T> = unsafe { ptr.as_ref() };

	match result {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}
    }

    #[test]
    fn as_ref_test2() {
	let x: T = 68;
	let ptr: *const T = &x;
	let result: Option<&T> = unsafe { ptr.as_ref() };

	match result {
	    Some(v) => assert_eq!(*v, x),
	    None => assert!(false)
	}
    }
}
