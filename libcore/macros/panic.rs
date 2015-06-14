#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! panic {
    //     () => (
    //         panic!("explicit panic")
    //     );
    //     ($msg:expr) => ({
    //         static _MSG_FILE_LINE: (&'static str, &'static str, u32) = ($msg, file!(), line!());
    //         ::core::panicking::panic(&_MSG_FILE_LINE)
    //     });
    //     ($fmt:expr, $($arg:tt)*) => ({
    //         // The leading _'s are to avoid dead code warnings if this is
    //         // used inside a dead function. Just `#[allow(dead_code)]` is
    //         // insufficient, since the user may have
    //         // `#[forbid(dead_code)]` and which cannot be overridden.
    //         static _FILE_LINE: (&'static str, u32) = (file!(), line!());
    //         ::core::panicking::panic_fmt(format_args!($fmt, $($arg)*), &_FILE_LINE)
    //     });
    // }

    #[test]
    #[should_panic]
    fn panic_test1() {
	panic!(); // panicked at 'explicit panic'
    }

    #[test]
    #[should_panic]
    fn panic_test2() {
	panic!("Hello, World!"); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn panic_test3() {
	panic!("{}, {}!", "Hello", "World"); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn panic_test4() {
	let arg1: &'static str = "Hello";
	let arg2: &'static str = "World";

	panic!("{}, {}!", arg1, arg2); // panicked at 'Hello, World!'
    }

    #[test]
    #[should_panic]
    fn panic_test5() {
	let arg1: usize = 68;
	let arg2: usize = 500;

	panic!("arg1 = [{}], arg2 = [{}]", arg1, arg2); // panicked at 'arg1 = [68], arg2 = [500]'
    }
}
