#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl<T> Pointer for *const T {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         let old_width = f.width;
    //         let old_flags = f.flags;
    //
    //         // The alternate flag is already treated by LowerHex as being special-
    //         // it denotes whether to prefix with 0x. We use it to work out whether
    //         // or not to zero extend, and then unconditionally set it to get the
    //         // prefix.
    //         if f.flags & 1 << (FlagV1::Alternate as u32) > 0 {
    //             f.flags |= 1 << (FlagV1::SignAwareZeroPad as u32);
    //
    //             if let None = f.width {
    //                 // The formats need two extra bytes, for the 0x
    //                 if cfg!(target_pointer_width = "32") {
    //                     f.width = Some(10);
    //                 } else {
    //                     f.width = Some(18);
    //                 }
    //             }
    //         }
    //         f.flags |= 1 << (FlagV1::Alternate as u32);
    //
    //         let ret = LowerHex::fmt(&(*self as usize), f);
    //
    //         f.width = old_width;
    //         f.flags = old_flags;
    //
    //         ret
    //     }
    // }

    // impl<T> Pointer for *mut T {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         // FIXME(#23542) Replace with type ascription.
    //         #![allow(trivial_casts)]
    //         Pointer::fmt(&(*self as *const T), f)
    //     }
    // }

    // impl<T> Debug for *mut T {
    //     fn fmt(&self, f: &mut Formatter) -> Result { Pointer::fmt(self, f) }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:p}", value_ptrmut); // "0x101606c7c"
    }

    #[test]
    fn fmt_test2() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:#p}", value_ptrmut); // "0x000000010ee00c7c"
    }

    #[test]
    fn fmt_test3() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:#20p}", value_ptrmut); // "0x00000000010cc02c7c"
    }

    #[test]
    fn fmt_test4() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:#25p}", value_ptrmut); // "0x00000000000000103003c7c"
    }

    #[test]
    fn fmt_test5() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:?}", value_ptrmut); // "0x10e003c7c"
    }

    #[test]
    fn fmt_test6() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:#?}", value_ptrmut); // "0x0000000106e00c7c"
    }

    #[test]
    fn fmt_test7() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:#20?}", value_ptrmut); // "0x000000000111003c7c"
    }

    #[test]
    fn fmt_test8() {
	let mut value: T = 68;
	let value_ptrmut: *mut T = &mut value;
	let _: String = format!("{:#25?}", value_ptrmut); // "0x00000000000000114600c7c"
    }
}
