#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //    impl PartialOrd for str {
    //        #[inline]
    //        fn partial_cmp(&self, other: &str) -> Option<Ordering> {
    //            Some(self.cmp(other))
    //        }
    //    }

    #[test]
    fn partial_cmp_test1() {
	let x: &str = "日";	// '\u{65e5}'
	let other: &str = "月";	// '\u{6708}'
	let result: Option<Ordering> = x.partial_cmp(other);

	assert_eq!(result, Some::<Ordering>(Less));
    }

    #[test]
    fn partial_cmp_test2() {
	let x: &str = "天";	// '\u{5929}'
	let other: &str = "地";	// '\u{5730}'
	let result: Option<Ordering> = x.partial_cmp(other);

	assert_eq!(result, Some::<Ordering>(Greater));
    }

    #[test]
    fn partial_cmp_test3() {
	let x: &str = "人";
	let other: &str = x;
	let result: Option<Ordering> = x.partial_cmp(other);

	assert_eq!(result, Some::<Ordering>(Equal));
    }

    #[test]
    fn partial_cmp_test4() {
	let x: &str = "人口";
	let other: &str = "人";
	let result: Option<Ordering> = x.partial_cmp(other);

	assert_eq!(result, Some::<Ordering>(Greater));
    }

    #[test]
    fn partial_cmp_test5() {
	let x: &str = "人";
	let other: &str = "人種";
	let result: Option<Ordering> = x.partial_cmp(other);

	assert_eq!(result, Some::<Ordering>(Less));
    }
}
