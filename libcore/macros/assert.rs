#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! assert {
    //     ($cond:expr) => (
    //         if !$cond {
    //             panic!(concat!("assertion failed: ", stringify!($cond)))
    //         }
    //     );
    //     ($cond:expr, $($arg:tt)+) => (
    //         if !$cond {
    //             panic!($($arg)+)
    //         }
    //     );
    // }

    #[test]
    fn assert_test1() {
	assert!(true);
    }

    #[test]
    #[should_panic]
    fn assert_test2() {
	assert!(false); // panicked at 'Some tests failed'
    }

    #[test]
    #[should_panic]
    fn assert_test3() {
	assert!(68 == 500); // panicked at 'assertion failed: 68 == 500'
    }

    #[test]
    #[should_panic]
    fn assert_test4() {
	let expr: &'static str = "Hello, World!";
	assert!(68 == 500, expr); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn assert_test5() {
	assert!(68 == 500, "{}, {}!", "Hello", "World"); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn assert_test6() {
	let arg1: &'static str = "Hello";
	let arg2: &'static str = "World";

	assert!(68 == 500, "{}, {}!", arg1, arg2); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn assert_test7() {
	let arg1: usize = 68;
	let arg2: usize = 500;

	assert!(arg1 == arg2, "arg1 = [{}], arg2= = [{}]", arg1, arg2); // panicked at 'arg1 = [68], arg2= = [500]'
    }
}
