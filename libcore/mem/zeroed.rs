#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::zeroed;

    // pub unsafe fn zeroed<T>() -> T {
    //     intrinsics::init()
    // }


    #[test]
    fn zeroed_test1() {
	let buf: [u8; 8] = unsafe { zeroed::<[u8; 8]>() };

	assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
