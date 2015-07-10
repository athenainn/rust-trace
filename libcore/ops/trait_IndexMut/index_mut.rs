#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Index;
    use core::ops::IndexMut;

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

    impl IndexMut<usize> for Pixel<T> {
	fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
	    match index {
		0     => &mut self.r,
		1     => &mut self.g,
		2 | _ => &mut self.b
	    }
	}
    }

    type T = u8;

    #[test]
    fn index_test1() {
	let mut pixel: Pixel<T> = Pixel { r: 0xff, g: 0x92, b: 0x24 };

	pixel[0] = 0x01;
	pixel[1] = 0x98;
	pixel[2] = 0x58;

	assert_eq!(pixel[0], 0x01);
	assert_eq!(pixel[1], 0x98);
	assert_eq!(pixel[2], 0x58);
	assert_eq!(pixel[3], 0x58);
    }
}
