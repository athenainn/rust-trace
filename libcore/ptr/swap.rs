#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::swap;

    // pub unsafe fn swap<T>(x: *mut T, y: *mut T) {
    //     // Give ourselves some scratch space to work with
    //     let mut tmp: T = mem::uninitialized();
    //
    //     // Perform the swap
    //     copy_nonoverlapping(x, &mut tmp, 1);
    //     copy(y, x, 1); // `x` and `y` may overlap
    //     copy_nonoverlapping(&tmp, y, 1);
    //
    //     // y and t now point to the same thing, but we need to completely forget `tmp`
    //     // because it's no longer relevant.
    //     mem::forget(tmp);
    // }

    type T = i32;

    #[test]
    fn swap_test1() {
	let mut x: T = 68;
	let mut y: T = 500;

	unsafe {
	    swap::<T>(&mut x, &mut y);
	}

	assert_eq!(x, 500);
	assert_eq!(y, 68);
    }
}
