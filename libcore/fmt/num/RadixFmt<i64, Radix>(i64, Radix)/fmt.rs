#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::radix;
    use core::fmt::RadixFmt;
    use core::fmt::Radix;

    // #[derive(Clone, Copy, PartialEq)]
    // #[unstable(feature = "core",
    //            reason = "may be renamed or move to a different module")]
    // pub struct Radix {
    //     base: u8,
    // }

    // #[derive(Copy, Clone)]
    // pub struct RadixFmt<T, R>(T, R);

    // pub fn radix<T>(x: T, base: u8) -> RadixFmt<T, Radix> {
    //     RadixFmt(x, Radix::new(base))
    // }

    // macro_rules! radix_fmt {
    //     ($T:ty as $U:ty, $fmt:ident) => {
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl fmt::Debug for RadixFmt<$T, Radix> {
    //             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //                 fmt::Display::fmt(self, f)
    //             }
    //         }
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl fmt::Display for RadixFmt<$T, Radix> {
    //             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //                 match *self { RadixFmt(ref x, radix) => radix.$fmt(*x as $U, f) }
    //             }
    //         }
    //     }
    // }

    // macro_rules! int_base {
    //     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl fmt::$Trait for $T {
    //             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //                 $Radix.fmt_int(*self as $U, f)
    //             }
    //         }
    //     }
    // }

    // macro_rules! debug {
    //     ($T:ident) => {
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl fmt::Debug for $T {
    //             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //                 fmt::Display::fmt(self, f)
    //             }
    //         }
    //     }
    // }

    // macro_rules! integer {
    //     ($Int:ident, $Uint:ident) => {
    //         int_base! { Display  for $Int as $Int   -> Decimal }
    //         int_base! { Binary   for $Int as $Uint  -> Binary }
    //         int_base! { Octal    for $Int as $Uint  -> Octal }
    //         int_base! { LowerHex for $Int as $Uint  -> LowerHex }
    //         int_base! { UpperHex for $Int as $Uint  -> UpperHex }
    //         radix_fmt! { $Int as $Int, fmt_int }
    //         debug! { $Int }
    //
    //         int_base! { Display  for $Uint as $Uint -> Decimal }
    //         int_base! { Binary   for $Uint as $Uint -> Binary }
    //         int_base! { Octal    for $Uint as $Uint -> Octal }
    //         int_base! { LowerHex for $Uint as $Uint -> LowerHex }
    //         int_base! { UpperHex for $Uint as $Uint -> UpperHex }
    //         radix_fmt! { $Uint as $Uint, fmt_int }
    //         debug! { $Uint }
    //     }
    // }
    // integer! { isize, usize }
    // integer! { i8, u8 }
    // integer! { i16, u16 }
    // integer! { i32, u32 }
    // integer! { i64, u64 }

    macro_rules! radix_fmt_test_impl {
	($T:ty, $value:expr, $base:expr, $s:expr) => (
	    {
		let x: $T = $value;
		let base: u8 = $base;
		let radixfmt: RadixFmt<$T, Radix> = radix::<$T>(x, base);

		let output: String = format!("{:?}", radixfmt);
		assert_eq!(output, $s.to_string());

		let output: String = format!("{}", radixfmt);
		assert_eq!(output, $s.to_string());
	    }
	)
    }

    macro_rules! radix_fmt_test {
	() => {	
	    radix_fmt_test_impl! { i64, 68, 2, "1000100" }
	    radix_fmt_test_impl! { i64, 68, 3, "2112" }
	    radix_fmt_test_impl! { i64, 68, 4, "1010" }
	    radix_fmt_test_impl! { i64, 68, 5, "233" }
	    radix_fmt_test_impl! { i64, 68, 6, "152" }
	    radix_fmt_test_impl! { i64, 68, 7, "125" }
	    radix_fmt_test_impl! { i64, 68, 8, "104" }
	    radix_fmt_test_impl! { i64, 68, 9, "75" }
	    radix_fmt_test_impl! { i64, 68, 10, "68" }
	    radix_fmt_test_impl! { i64, 68, 11, "62" }
	    radix_fmt_test_impl! { i64, 68, 12, "58" }
	    radix_fmt_test_impl! { i64, 68, 13, "53" }
	    radix_fmt_test_impl! { i64, 68, 14, "4c" }
	    radix_fmt_test_impl! { i64, 68, 15, "48" }
	    radix_fmt_test_impl! { i64, 68, 16, "44" }
	    radix_fmt_test_impl! { i64, 68, 17, "40" }
	    radix_fmt_test_impl! { i64, 68, 18, "3e" }
	    radix_fmt_test_impl! { i64, 68, 19, "3b" }
	    radix_fmt_test_impl! { i64, 68, 20, "38" }
	    radix_fmt_test_impl! { i64, 68, 21, "35" }
	    radix_fmt_test_impl! { i64, 68, 22, "32" }
	    radix_fmt_test_impl! { i64, 68, 23, "2m" }
	    radix_fmt_test_impl! { i64, 68, 24, "2k" }
	    radix_fmt_test_impl! { i64, 68, 25, "2i" }
	    radix_fmt_test_impl! { i64, 68, 26, "2g" }
	    radix_fmt_test_impl! { i64, 68, 27, "2e" }
	    radix_fmt_test_impl! { i64, 68, 28, "2c" }
	    radix_fmt_test_impl! { i64, 68, 29, "2a" }
	    radix_fmt_test_impl! { i64, 68, 30, "28" }
	    radix_fmt_test_impl! { i64, 68, 31, "26" }
	    radix_fmt_test_impl! { i64, 68, 32, "24" }
	    radix_fmt_test_impl! { i64, 68, 33, "22" }
	    radix_fmt_test_impl! { i64, 68, 34, "20" }
	    radix_fmt_test_impl! { i64, 68, 35, "1x" }
	    radix_fmt_test_impl! { i64, 68, 36, "1w" }
	}
    }

    #[test]
    #[should_panic]
    fn fmt_test1() {
	radix_fmt_test_impl! { i64, 68, 1, "" }; // panicked at 'the base must be in the range of 2..36: 1'
    }

    #[test]
    fn fmt_test2() {
	radix_fmt_test!();
    }

    #[test]
    #[should_panic]
    fn fmt_test3() {
	radix_fmt_test_impl! { i64, 68, 37, "" }; // panicked at 'the base must be in the range of 2..36: 37'
    }

}
