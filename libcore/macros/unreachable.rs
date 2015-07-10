#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! unreachable {
    //     () => ({
    //         panic!("internal error: entered unreachable code")
    //     });
    //     ($msg:expr) => ({
    //         unreachable!("{}", $msg)
    //     });
    //     ($fmt:expr, $($arg:tt)*) => ({
    //         panic!(concat!("internal error: entered unreachable code: ", $fmt), $($arg)*)
    //     });
    // }

    #[test]
    #[should_panic]
    fn unreachable_test1() {
	unreachable!(); // panicked at 'internal error: entered unreachable code'
    }

    #[test]
    #[should_panic]
    fn unreachable_test2() {
	unreachable!("Hello, World!"); // panicked at 'internal error: entered unreachable code: Hello, World!'
    }

    #[test]
    #[should_panic]
    fn unreachable_test3() {
	unreachable!("{}, {}!", "Hello", "World"); // panicked at 'internal error: entered unreachable code: Hello, World!'
    }

    #[test]
    #[should_panic]
    fn unreachable_test4() {
	let arg1: &'static str = "Hello";
	let arg2: &'static str = "World";

	unreachable!("{}, {}!", arg1, arg2); // panicked at 'internal error: entered unreachable code: Hello, World!'
    }

    #[test]
    #[should_panic]
    fn unreachable_test5() {
	let arg1: usize = 68;
	let arg2: usize = 500;

	unreachable!("arg1 = [{}], arg2 = [{}]", arg1, arg2); // panicked at 'internal error: entered unreachable code: 	thread 'tests::unreachable_test5' panicked at 'internal error: entered unreachable code: arg1 = [68], arg2 = [500]'
    }
}
