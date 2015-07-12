#![feature(alloc, heap_api)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::heap::stats_print;

    // pub fn stats_print() {
    //     imp::stats_print();
    // }

    #[test]
    fn stats_print_test1() {
	stats_print();
    }
}
