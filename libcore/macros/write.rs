#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::fmt;
    use core::fmt::Write;

    use collections::vec::Vec;

    // macro_rules! write {
    //     ($dst:expr, $($arg:tt)*) => ($dst.write_fmt(format_args!($($arg)*)))
    // }

    struct A {
	buf: Vec<u8>
    }

    impl A {
	fn new() -> A {
	    A { buf: vec!() }
	}
    }

    impl fmt::Write for A {
	fn write_str(&mut self, s: &str) -> fmt::Result {
	    for b in s.bytes() {
		self.buf.push(b);
	    }
	    Ok(())
	}

	// fn write_fmt(&mut self, args: Arguments) -> Result {
	//     // This Adapter is needed to allow `self` (of type `&mut
	//     // Self`) to be cast to a Write (below) without
	//     // requiring a `Sized` bound.
	//     struct Adapter<'a,T: ?Sized +'a>(&'a mut T);
	//
	//     impl<'a, T: ?Sized> Write for Adapter<'a, T>
	//         where T: Write
	//     {
	//         fn write_str(&mut self, s: &str) -> Result {
	//             self.0.write_str(s)
	//         }
	//
	//         fn write_fmt(&mut self, args: Arguments) -> Result {
	//             self.0.write_fmt(args)
	//         }
	//     }
	//
	//     write(&mut Adapter(self), args)
	// }

    }

    #[test]
    fn write_test1() {
	let mut a: A = A::new();
	let result: fmt::Result = write!(a, "Hello, World!"); // invoked A::write_fmt

	match result {
	    Ok(()) => assert!(true),
	    Err(fmt::Error) => assert!(false)
	}

	assert_eq!(a.buf, "Hello, World!".as_bytes());
    }

    #[test]
    fn write_test2() {
	let mut a: A = A::new();
	let result: fmt::Result = write!(a, "{}, {}!", "Hello", "World"); // invoked A::write_fmt

	match result {
	    Ok(()) => assert!(true),
	    Err(fmt::Error) => assert!(false)
	}

	assert_eq!(a.buf, "Hello, World!".as_bytes());
    }

    #[test]
    fn write_test3() {
	let mut a: A = A::new();
	let arg1: &'static str = "Hello";
	let arg2: &'static str = "World";
	let result: fmt::Result = write!(a, "{}, {}!", arg1, arg2); // invoked A::write_fmt

	match result {
	    Ok(()) => assert!(true),
	    Err(fmt::Error) => assert!(false)
	}

	assert_eq!(a.buf, "Hello, World!".as_bytes());
    }

    #[test]
    fn write_test4() {
	let mut a: A = A::new();
	let arg1: usize = 68;
	let arg2: usize = 500;
	let result: fmt::Result = write!(a, "arg1 = [{}], arg2 = [{}]", arg1, arg2); // invoked A::write_fmt

	match result {
	    Ok(()) => assert!(true),
	    Err(fmt::Error) => assert!(false)
	}

	assert_eq!(a.buf, "arg1 = [68], arg2 = [500]".as_bytes());
    }
}
