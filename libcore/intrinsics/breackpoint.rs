#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::breakpoint;

    // pub fn breakpoint();

    #[test]
    fn breakpoint_test1() {
	unsafe { breakpoint(); }
    }
}
