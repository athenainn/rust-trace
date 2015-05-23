#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Index;

    struct Pixel<T> {
	r: T,
	g: T,
	b: T
    }

    impl Index<usize> for Pixel<T> {
	type Output = T;

	fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
	    match index {
		0     => &self.r,
		1     => &self.g,
		2 | _ => &self.b
	    }
	}
    }

    type T = u8;

    #[test]
    fn index_test1() {
	let pixel: Pixel<T> = Pixel { r: 0xff, g: 0x92, b: 0x24 };

	assert_eq!(pixel[0], 0xff);
	assert_eq!(pixel[1], 0x92);
	assert_eq!(pixel[2], 0x24);
	assert_eq!(pixel[3], 0x24);
    }
}
