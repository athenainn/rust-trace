#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     pub unsafe fn as_mut<'a>(&self) -> Option<&'a mut T> where T: Sized {
    //         if self.is_null() {
    //             None
    //         } else {
    //             Some(&mut **self)
    //         }
    //     }

    type T = i32;

    #[test]
    fn as_mut_test1() {
	let ptr: *mut T = 0 as *mut T;
	let result: Option<&mut T> = unsafe { ptr.as_mut() };

	match result {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}
    }

    #[test]
    fn as_mut_test2() {
	let mut x: T = 68;
	let ptr: *mut T = &mut x;
	let result: Option<&mut T> = unsafe { ptr.as_mut() };

	match result {
	    Some(v) => { assert_eq!(*v, x); *v = 500 },
	    None => assert!(false)
	}

	assert_eq!(x, 500);
    }
}
