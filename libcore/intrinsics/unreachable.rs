#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::unreachable;

    // pub fn unreachable() -> !;

    #[test]
    fn unreachable_test1() {
	unreachable();
    }
}
