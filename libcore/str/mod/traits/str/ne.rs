#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     impl PartialEq for str {
    //         #[inline]
    //         fn eq(&self, other: &str) -> bool {
    //             eq_slice(self, other)
    //         }
    //         #[inline]
    //         fn ne(&self, other: &str) -> bool { !(*self).eq(other) }
    //     }

    #[test]
    fn ne_test1() {
	let x: &str = "天地玄黃";
	let other: &str = "天地玄黃";
	let result: bool = x.ne(other);

	assert_eq!(result, false);
    }

    #[test]
    fn ne_test2() {
	let x: &str = "宇宙洪荒";
	let other: &str = "宇宙洪荒";
	let result: bool = x != other;

	assert_eq!(result, false);
    }

}
