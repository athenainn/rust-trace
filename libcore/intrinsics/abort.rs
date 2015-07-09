#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::abort;

    // pub fn abort() -> !;

    #[test]
    fn abort_test1() {
	unsafe { abort(); }
    }
}
