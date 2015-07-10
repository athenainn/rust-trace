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

    // impl<T> Debug for *const T {
    //     fn fmt(&self, f: &mut Formatter) -> Result { Pointer::fmt(self, f) }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:p}", value_ptr); // "0x10ba00c7c"
    }

    #[test]
    fn fmt_test2() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:#p}", value_ptr); // "0x0000000103e00c7c"
    }

    #[test]
    fn fmt_test3() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:#20p}", value_ptr); // "0x000000000110600c7c"
    }

    #[test]
    fn fmt_test4() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:#25p}", value_ptr); // "0x00000000000000106e00c7c"
    }

    #[test]
    fn fmt_test5() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:?}", value_ptr); // "0x109206c7c"
    }

    #[test]
    fn fmt_test6() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:#?}", value_ptr); // "0x000000010d226c7c"
    }

    #[test]
    fn fmt_test7() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:#20?}", value_ptr); // "0x00000000010fe06c7c"
    }

    #[test]
    fn fmt_test8() {
	let value: T = 68;
	let value_ptr: *const T = &value;
	let _: String = format!("{:#25?}", value_ptr); // "0x0000000000000010aa8ec7c"
    }
}
