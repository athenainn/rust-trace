#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::default::Default;

    struct A {
	value: u32
    }

    impl Default for A {
	fn default() -> Self {
	    A { value: 68 }
	}
    }

    #[derive(Debug, PartialEq)]
    enum Kind {
	A,
	B,
	C
    }

    impl Default for Kind {
	fn default() -> Kind {
	    Kind::A
	}
    }

    #[test]
    fn default_test1() {
	let a: A = A::default();
	assert_eq!(a.value, 68);
    }

    #[test]
    fn default_test2() {
	let a: A = Default::default();
	assert_eq!(a.value, 68);
    }

    #[test]
    fn default_test3() {
	let a: A = Default::default();
	assert_eq!(a.value, 68);
    }

    #[test]
    fn default_test4() {
	let (x, y): (Option<String>, f64) = Default::default();
	assert_eq!(x, None::<String>);
	assert_eq!(y, 0.0);
    }

    #[test]
    fn default_test5() {
	let (a, b, (c, d)): (i32, u32, (bool, bool)) = Default::default();
	assert_eq!(a, 0);
	assert_eq!(b, 0);
	assert_eq!(c, false);
	assert_eq!(d, false);
    }

    #[test]
    fn default_test6() {
	assert_eq!(<()>::default(), ());
	assert_eq!(bool::default(), false);
	assert_eq!(char::default(), '\x00');
	assert_eq!(usize::default(), 0);
	assert_eq!(u8::default(), 0);
	assert_eq!(u16::default(), 0);
	assert_eq!(u32::default(), 0);
	assert_eq!(u64::default(), 0);
	assert_eq!(isize::default(), 0);
	assert_eq!(i8::default(), 0);
	assert_eq!(i16::default(), 0);
	assert_eq!(i32::default(), 0);
	assert_eq!(i64::default(), 0);
	assert_eq!(f32::default(), 0.0f32);
	assert_eq!(f64::default(), 0.0f64);
    }
    #[test]
    fn default_test7() {
	let x: Kind = Default::default();
	let y: Kind = Kind::default();

	assert_eq!(x, Kind::A);
	assert_eq!(y, Kind::A);
    }
}
