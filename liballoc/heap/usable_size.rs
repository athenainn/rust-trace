#![feature(alloc, heap_api)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::heap::usable_size;
    use alloc::heap::allocate;
    use alloc::heap::deallocate;

    // pub fn usable_size(size: usize, align: usize) -> usize {
    //     imp::usable_size(size, align)
    // }

    type T = u16;

    macro_rules! usable_size_test {
	($size:expr, $align:expr, $usable:expr) => ({
	    let size: usize = $size; // != 0
	    let align: usize = $align; // power of 2
	    let ptr: *mut u8 = unsafe { allocate(size, align) };
	    assert!(ptr != 0 as *mut u8);

	    let usable: usize = usable_size(size, align);
	    assert_eq!(usable, $usable);

	    let old_size: usize = size;
	    unsafe { deallocate(ptr, old_size, align); }
	})
    }

    #[test]
    fn usable_size_test1() {
	usable_size_test!( 8, 8, 8 );
	usable_size_test!( 16, 8, 16 );
	usable_size_test!( 32, 8, 32 );
	usable_size_test!( 48, 8, 48 );
	usable_size_test!( 64, 8, 64 );
	usable_size_test!( 80, 8, 80 );
	usable_size_test!( 96, 8, 96 );
	usable_size_test!( 112, 8, 112 );
	usable_size_test!( 128, 8, 128 );
	usable_size_test!( 160, 8, 160 );
	usable_size_test!( 192, 8, 192 );
	usable_size_test!( 224, 8, 224 );
	usable_size_test!( 256, 8, 256 );
	usable_size_test!( 320, 8, 320 );
	usable_size_test!( 384, 8, 384 );
	usable_size_test!( 448, 8, 448 );
	usable_size_test!( 512, 8, 512 );
	usable_size_test!( 640, 8, 640 );
	usable_size_test!( 768, 8, 768 );
	usable_size_test!( 896, 8, 896 );
	usable_size_test!( 1024, 8, 1024 );
	usable_size_test!( 1280, 8, 1280 );
	usable_size_test!( 1536, 8, 1536 );
	usable_size_test!( 1792, 8, 1792 );
	usable_size_test!( 2048, 8, 2048 );
	usable_size_test!( 2560, 8, 2560 );
	usable_size_test!( 3072, 8, 3072 );
	usable_size_test!( 3584, 8, 3584 );
	usable_size_test!( 4096, 8, 4096 );
	usable_size_test!( 8192, 8, 8192 );
	usable_size_test!( 12288, 8, 12288 );
	usable_size_test!( 16384, 8, 16384 );
	usable_size_test!( 20480, 8, 20480 );
	usable_size_test!( 24576, 8, 24576 );
	usable_size_test!( 28672, 8, 28672 );
	usable_size_test!( 32768, 8, 32768 );
	usable_size_test!( 36864, 8, 36864 );
	usable_size_test!( 40960, 8, 40960 );
	usable_size_test!( 45056, 8, 45056 );
	usable_size_test!( 49152, 8, 49152 );
	usable_size_test!( 53248, 8, 53248 );
	usable_size_test!( 57344, 8, 57344 );
	usable_size_test!( 61440, 8, 61440 );
	usable_size_test!( 65536, 8, 65536 );
	usable_size_test!( 69632, 8, 69632 );
	usable_size_test!( 73728, 8, 73728 );
	usable_size_test!( 77824, 8, 77824 );
	usable_size_test!( 81920, 8, 81920 );
	usable_size_test!( 86016, 8, 86016 );
	usable_size_test!( 90112, 8, 90112 );
	usable_size_test!( 94208, 8, 94208 );
	usable_size_test!( 98304, 8, 98304 );
	usable_size_test!( 102400, 8, 102400 );
	usable_size_test!( 106496, 8, 106496 );
	usable_size_test!( 110592, 8, 110592 );
	usable_size_test!( 114688, 8, 114688 );
	usable_size_test!( 118784, 8, 118784 );
	usable_size_test!( 122880, 8, 122880 );
	usable_size_test!( 126976, 8, 126976 );
	usable_size_test!( 131072, 8, 131072 );
	usable_size_test!( 135168, 8, 135168 );
	usable_size_test!( 139264, 8, 139264 );
	usable_size_test!( 143360, 8, 143360 );
	usable_size_test!( 147456, 8, 147456 );
	usable_size_test!( 151552, 8, 151552 );
	usable_size_test!( 155648, 8, 155648 );
	usable_size_test!( 159744, 8, 159744 );
	usable_size_test!( 163840, 8, 163840 );
    }
}
