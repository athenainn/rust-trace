#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::swap;

    // pub unsafe fn swap<T>() -> T {
    //     intrinsics::uninit()
    // }

    macro_rules! swap_test {
	($T:ty, $x:expr, $y:expr) => ({
	    let mut x: $T = $x;
	    let mut y: $T = $y;

	    swap::<$T>(&mut x, &mut y);

	    assert_eq!(x, $y as $T);
	    assert_eq!(y, $x as $T);
	})
    }

    #[test]
    fn swap_test1() {
	swap_test!( u8, 68, 99 );
	swap_test!( u16, 68, 99 );
	swap_test!( u32, 68, 99 );
	swap_test!( u64, 68, 99 );
	swap_test!( usize, 68, 99 );

	swap_test!( i8, 68, 99 );
	swap_test!( i16, 68, 99 );
	swap_test!( i32, 68, 99 );
	swap_test!( i64, 68, 99 );
	swap_test!( isize, 68, 99 );

	swap_test!( f32, 68.0, 99.0 );
	swap_test!( f64, 68.0, 99.0 );
    }

    #[test]
    fn swap_test2() {
	swap_test!( [u8; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [u16; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [u32; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [u64; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [usize; 3], [1, 2, 3], [4, 5, 6]);

	swap_test!( [i8; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [i16; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [i32; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [i64; 3], [1, 2, 3], [4, 5, 6]);
	swap_test!( [isize; 3], [1, 2, 3], [4, 5, 6]);

	swap_test!( [f32; 3], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
	swap_test!( [f64; 3], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    }
}
