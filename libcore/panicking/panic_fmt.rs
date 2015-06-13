#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::panicking::panic_fmt;

    use core::fmt;

    // pub fn panic_fmt(fmt: fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
    //     #[allow(improper_ctypes)]
    //     extern {
    //         #[lang = "panic_fmt"]
    //         fn panic_impl(fmt: fmt::Arguments, file: &'static str, line: u32) -> !;
    //     }
    //     let (file, line) = *file_line;
    //     unsafe { panic_impl(fmt, file, line) }
    // }

    #[test]
    #[should_panic]
    fn panic_fmt_test1() {
	let expr: &'static str = "Hello, World!";
	let pieces: &[&'static str]  = &[expr];
	let args: &[fmt::ArgumentV1] = &[];
	let fmt: fmt::Arguments = fmt::Arguments::new_v1(pieces, args);

	let file: &'static str = "FILENAME";
	let line: u32 = 68;
	let file_line: (&'static str, u32) = (file, line);

	panic_fmt(fmt, &file_line); // panicked at 'Hello, World!', FILENAME:68
    }

    #[test]
    #[should_panic]
    fn panic_fmt_test2() {
	let pieces: &[&'static str]  = &["", ", ", "!"];

	let x1: String = "Hello".to_string();
	let x2: String = "World".to_string();
	let args: &[fmt::ArgumentV1] = &[
	    fmt::ArgumentV1::new(&x1, fmt::Display::fmt),
	    fmt::ArgumentV1::new(&x2, fmt::Display::fmt)
	];

	let fmt: fmt::Arguments = fmt::Arguments::new_v1(pieces, args);

	let file: &'static str = "FILENAME";
	let line: u32 = 68;
	let file_line: (&'static str, u32) = (file, line);

	panic_fmt(fmt, &file_line); // panicked at 'Hello, World!', FILENAME:68
    }

    #[test]
    #[should_panic]
    fn panic_fmt_test3() {
	let pieces: &[&'static str]  = &["arg1 = [", "], arg2 = [", "]"];

	let x1: usize = 500;
	let x2: usize = 999;
	let args: &[fmt::ArgumentV1] = &[
	    fmt::ArgumentV1::new(&x1, fmt::Display::fmt),
	    fmt::ArgumentV1::new(&x2, fmt::Display::fmt)
	];

	let fmt: fmt::Arguments = fmt::Arguments::new_v1(pieces, args);

	let file: &'static str = "FILENAME";
	let line: u32 = 68;
	let file_line: (&'static str, u32) = (file, line);

	panic_fmt(fmt, &file_line); // panicked at 'arg1 = [500], arg2 = [999]', FILENAME:68
    }
}
