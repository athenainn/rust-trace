#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
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

    macro_rules! fmt_test {
	($T:ty, $value:expr, $b:expr, $o:expr, $d:expr, $lh:expr, $uh:expr) => (
	    {
		let x: $T = $value;

		let output: String = format!("{}", x);
		assert_eq!(output, $d.to_string());

		let output: String = format!("{:?}", x);
		assert_eq!(output, $d.to_string());

		let output: String = format!("{:b}", x);
		assert_eq!(output, $b.to_string());

		let output: String = format!("{:#b}", x);
		assert_eq!(output, concat!("0b", $b).to_string());

		let output: String = format!("{:o}", x);
		assert_eq!(output, $o.to_string());

		let output: String = format!("{:#o}", x);
		assert_eq!(output, concat!("0o", $o).to_string());

		let output: String = format!("{:x}", x);
		assert_eq!(output, $lh.to_string());

		let output: String = format!("{:#x}", x);
		assert_eq!(output, concat!("0x", $lh).to_string());

		let output: String = format!("{:X}", x);
		assert_eq!(output, $uh.to_string());

		let output: String = format!("{:#X}", x);
		assert_eq!(output, concat!("0x", $uh).to_string());
	    }
	)
    }

    #[test]
    fn fmt_test1() {
	fmt_test! { u16, 500, "111110100", "764", "500", "1f4", "1F4" };
    }
}
