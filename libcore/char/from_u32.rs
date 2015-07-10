#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::char::from_u32;

    // pub fn from_u32(i: u32) -> Option<char> {
    //     // catch out-of-bounds and surrogates
    //     if (i > MAX as u32) || (i >= 0xD800 && i <= 0xDFFF) {
    //         None
    //     } else {
    //         Some(unsafe { transmute(i) })
    //     }
    // }

    #[test]
    fn from_u32_test1() {
	let beer_mug: u32 = 0x1F37A;
	let result: Option<char> = from_u32(beer_mug);

	match result {
	    Some(v) => assert_eq!(v, '\u{1F37A}'),
	    None => assert!(false)
	}
    }
}
