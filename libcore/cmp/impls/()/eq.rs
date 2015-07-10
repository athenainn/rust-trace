#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     impl PartialEq for () {
    //         #[inline]
    //         fn eq(&self, _other: &()) -> bool { true }
    //         #[inline]
    //         fn ne(&self, _other: &()) -> bool { false }
    //     }

    #[test]
    fn eq_test1() {
	let v1: () = ();
	let v2: () = ();
	let result: bool = v1.eq(&v2);

	assert_eq!(result, true);
    }
}
