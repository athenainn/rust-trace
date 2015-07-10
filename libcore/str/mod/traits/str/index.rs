#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Range;
    use core::ops::RangeTo;
    use core::ops::RangeFrom;
    use core::ops::RangeFull;

    //     impl ops::Index<ops::Range<usize>> for str {
    //         type Output = str;
    //         #[inline]
    //         fn index(&self, index: ops::Range<usize>) -> &str {
    //             // is_char_boundary checks that the index is in [0, .len()]
    //             if index.start <= index.end &&
    //                self.is_char_boundary(index.start) &&
    //                self.is_char_boundary(index.end) {
    //                 unsafe { self.slice_unchecked(index.start, index.end) }
    //             } else {
    //                 super::slice_error_fail(self, index.start, index.end)
    //             }
    //         }
    //     }

    //     impl ops::Index<ops::RangeTo<usize>> for str {
    //         type Output = str;
    //
    //         #[inline]
    //         fn index(&self, index: ops::RangeTo<usize>) -> &str {
    //             // is_char_boundary checks that the index is in [0, .len()]
    //             if self.is_char_boundary(index.end) {
    //                 unsafe { self.slice_unchecked(0, index.end) }
    //             } else {
    //                 super::slice_error_fail(self, 0, index.end)
    //             }
    //         }
    //     }

    //     impl ops::Index<ops::RangeFrom<usize>> for str {
    //         type Output = str;
    //
    //         #[inline]
    //         fn index(&self, index: ops::RangeFrom<usize>) -> &str {
    //             // is_char_boundary checks that the index is in [0, .len()]
    //             if self.is_char_boundary(index.start) {
    //                 unsafe { self.slice_unchecked(index.start, self.len()) }
    //             } else {
    //                 super::slice_error_fail(self, index.start, self.len())
    //             }
    //         }
    //     }

    //     impl ops::Index<ops::RangeFull> for str {
    //         type Output = str;
    //
    //         #[inline]
    //         fn index(&self, _index: ops::RangeFull) -> &str {
    //             self
    //         }
    //     }

    #[test]
    fn index_test1() {
	let s: &str = "Löwe 老虎 Léopard";
	let range: Range<usize> = 6..12;
	let result: &str = &s[range];

	assert_eq!(result, "老虎");
    }

    #[test]
    #[should_panic]
    fn index_test2() {
	let s: &str = "Löwe 老虎 Léopard";
	let range: Range<usize> = 7..12;
	let _: &str = &s[range]; // panicked at 'index 7 and/or 12 in `Löwe 老虎 Léopard` do not lie on character boundary'
    }

    #[test]
    fn index_test3() {
	let s: &str = "Löwe 老虎 Léopard";
	let rangeto: RangeTo<usize> = ..12;
	let result: &str = &s[rangeto];

	assert_eq!(result, "Löwe 老虎");
    }

    #[test]
    #[should_panic]
    fn index_test4() {
	let s: &str = "Löwe 老虎 Léopard";
	let rangeto: RangeTo<usize> = ..11;
	let _: &str = &s[rangeto]; // panicked at 'index 0 and/or 11 in `Löwe 老虎 Léopard` do not lie on character boundary'
    }

    #[test]
    fn index_test5() {
	let s: &str = "Löwe 老虎 Léopard";
	let rangefrom: RangeFrom<usize> = 6..;
	let result: &str = &s[rangefrom];

	assert_eq!(result, "老虎 Léopard");
    }

    #[test]
    #[should_panic]
    fn index_test6() {
	let s: &str = "Löwe 老虎 Léopard";
	let rangefrom: RangeFrom<usize> = 7..;
	let _: &str = &s[rangefrom]; // panicked at 'index 7 and/or 21 in `Löwe 老虎 Léopard` do not lie on character boundary'
    }

    #[test]
    fn index_test7() {
	let s: &str = "Löwe 老虎 Léopard";
	let rangefull: RangeFull = ..;
	let result: &str = &s[rangefull];

	assert_eq!(result, "Löwe 老虎 Léopard");
    }
}
