#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::transmute;

    // pub fn transmute<T,U>(e: T) -> U;

    #[test]
    fn transmute_test1() {
	let v: &[u8] = unsafe { transmute::<&str, &[u8]>("L") };
	assert_eq!(v, [b'L']);
    }

    #[test]
    fn transmute_test2() {
	#[derive(Clone, Copy, PartialEq, Debug)]
	enum E {
	    Yes = 1,
	    No = -1
	};

	let x: E = E::No;
	let y: E = unsafe { transmute::<i8, E>(-(x as i8)) };

	assert_eq!(y, E::Yes);
    }
}
