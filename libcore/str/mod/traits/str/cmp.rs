#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     impl Ord for str {
    //         #[inline]
    //         fn cmp(&self, other: &str) -> Ordering {
    //             for (s_b, o_b) in self.bytes().zip(other.bytes()) {
    //                 match s_b.cmp(&o_b) {
    //                     Greater => return Greater,
    //                     Less => return Less,
    //                     Equal => ()
    //                 }
    //             }
    //
    //             self.len().cmp(&other.len())
    //         }
    //     }

    #[test]
    fn cmp_test1() {
	let x: &str = "日";	// '\u{65e5}'
	let other: &str = "月";	// '\u{6708}'
	let result: Ordering = x.cmp(other);

	assert_eq!(result, Less);
    }

    #[test]
    fn cmp_test2() {
	let x: &str = "天";	// '\u{5929}'
	let other: &str = "地";	// '\u{5730}'
	let result: Ordering = x.cmp(other);

	assert_eq!(result, Greater);
    }

    #[test]
    fn cmp_test3() {
	let x: &str = "人";
	let other: &str = x;
	let result: Ordering = x.cmp(other);

	assert_eq!(result, Equal);
    }

    #[test]
    fn cmp_test4() {
	let x: &str = "人口";
	let other: &str = "人";
	let result: Ordering = x.cmp(other);

	assert_eq!(result, Greater);
    }

    #[test]
    fn cmp_test5() {
	let x: &str = "人";
	let other: &str = "人種";
	let result: Ordering = x.cmp(other);

	assert_eq!(result, Less);
    }
}
