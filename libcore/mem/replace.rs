#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::mem::replace;

    use collections::vec::Vec;

    // pub fn replace<T>(dest: &mut T, mut src: T) -> T {
    //     swap(dest, &mut src);
    //     src
    // }

    macro_rules! replace_test {
	($T:ty, $dest:expr, $src:expr) => ({
	    let mut dest: $T = $dest;
	    let src: $T = $src;

	    let result = replace::<$T>(&mut dest, src);

	    assert_eq!(result, $dest);
	    assert_eq!(dest, $src);
	})
    }

    #[test]
    fn replace_test1() {
	replace_test!( u8, 68, 99 );
	replace_test!( u16, 68, 99 );
	replace_test!( u32, 68, 99 );
	replace_test!( u64, 68, 99 );
	replace_test!( usize, 68, 99 );

	replace_test!( i8, 68, 99 );
	replace_test!( i16, 68, 99 );
	replace_test!( i32, 68, 99 );
	replace_test!( i64, 68, 99 );
	replace_test!( isize, 68, 99 );

	replace_test!( f32, 68.0, 99.0 );
	replace_test!( f64, 68.0, 99.0 );
    }

    #[test]
    fn replace_test2() {
	replace_test!( [u8; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [u16; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [u32; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [u64; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [usize; 3], [1, 2, 3], [4, 5, 6]);

	replace_test!( [i8; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [i16; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [i32; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [i64; 3], [1, 2, 3], [4, 5, 6]);
	replace_test!( [isize; 3], [1, 2, 3], [4, 5, 6]);

	replace_test!( [f32; 3], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
	replace_test!( [f64; 3], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    }


    #[test]
    fn replace_test3() {
	replace_test!( Vec<i32>, vec!(1, 2, 3), vec!());
    }
}
