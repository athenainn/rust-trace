#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::panicking::panic;

    // pub fn panic(expr_file_line: &(&'static str, &'static str, u32)) -> ! {
    //     // Use Arguments::new_v1 instead of format_args!("{}", expr) to potentially
    //     // reduce size overhead. The format_args! macro uses str's Display trait to
    //     // write expr, which calls Formatter::pad, which must accommodate string
    //     // truncation and padding (even though none is used here). Using
    //     // Arguments::new_v1 may allow the compiler to omit Formatter::pad from the
    //     // output binary, saving up to a few kilobytes.
    //     let (expr, file, line) = *expr_file_line;
    //     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]), &(file, line))
    // }

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
    fn panic_test() {
	let expr: &'static str = "Hello, World!";
	let file: &'static str = "FILENAME";
	let line: u32 = 68;
	let expr_file_line: (&'static str, &'static str, u32) = (expr, file, line);

	panic(&expr_file_line); // panicked at 'Hello, World!', FILENAME:68
    }
}
