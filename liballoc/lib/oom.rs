#![feature(alloc, oom)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::oom;

    // pub fn oom() -> ! {
    //     // FIXME(#14674): This really needs to do something other than just abort
    //     //                here, but any printing done must be *guaranteed* to not
    //     //                allocate.
    //     unsafe { core::intrinsics::abort() }
    // }

    #[test]
    fn oom_test1() {
	oom();
    }
}
