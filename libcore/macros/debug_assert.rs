// runs rustc with `-C debug-assertions=off` that can disable `debug_assert!`

#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! debug_assert {
    //     ($($arg:tt)*) => (if cfg!(debug_assertions) { assert!($($arg)*); })
    // }

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
    fn debug_assert_test1() {
	debug_assert!(true);
    }

    #[test]
    #[should_panic]
    fn debug_assert_test2() {
	debug_assert!(false); // panicked at 'Some tests failed'
    }

    #[test]
    #[should_panic]
    fn debug_assert_test3() {
	debug_assert!(68 == 500); // panicked at 'assertion failed: 68 == 500'
    }

    #[test]
    #[should_panic]
    fn debug_assert_test4() {
	let expr: &'static str = "Hello, World!";
	debug_assert!(68 == 500, expr); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn debug_assert_test5() {
	debug_assert!(68 == 500, "{}, {}!", "Hello", "World"); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn debug_assert_test6() {
	let arg1: &'static str = "Hello";
	let arg2: &'static str = "World";

	debug_assert!(68 == 500, "{}, {}!", arg1, arg2); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn debug_assert_test7() {
	let arg1: usize = 68;
	let arg2: usize = 500;

	debug_assert!(arg1 == arg2, "arg1 = [{}], arg2= = [{}]", arg1, arg2); // panicked at 'arg1 = [68], arg2= = [500]'
    }
}
