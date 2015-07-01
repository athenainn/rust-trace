#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::strategy::grisu::max_pow10_no_more_than;

    // pub fn max_pow10_no_more_than(x: u32) -> (u8, u32) {
    //     debug_assert!(x > 0);
    //
    //     const X9: u32 = 10_0000_0000;
    //     const X8: u32 =  1_0000_0000;
    //     const X7: u32 =    1000_0000;
    //     const X6: u32 =     100_0000;
    //     const X5: u32 =      10_0000;
    //     const X4: u32 =       1_0000;
    //     const X3: u32 =         1000;
    //     const X2: u32 =          100;
    //     const X1: u32 =           10;
    //
    //     if x < X4 {
    //         if x < X2 { if x < X1 {(0,  1)} else {(1, X1)} }
    //         else      { if x < X3 {(2, X2)} else {(3, X3)} }
    //     } else {
    //         if x < X6      { if x < X5 {(4, X4)} else {(5, X5)} }
    //         else if x < X8 { if x < X7 {(6, X6)} else {(7, X7)} }
    //         else           { if x < X9 {(8, X8)} else {(9, X9)} }
    //     }
    // }

    type T = f32; // T: DecodableFloat

    #[test]
    fn max_pow10_no_more_than_test1() {
	let x: u32 = 1;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 0);
	assert_eq!(max_ten_kappa, 1);
    }

    #[test]
    fn max_pow10_no_more_than_test2() {
	let x: u32 = 12;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 1);
	assert_eq!(max_ten_kappa, 10);
    }

    #[test]
    fn max_pow10_no_more_than_test3() {
	let x: u32 = 123;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 2);
	assert_eq!(max_ten_kappa, 100);
    }

    #[test]
    fn max_pow10_no_more_than_test4() {
	let x: u32 = 1234;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 3);
	assert_eq!(max_ten_kappa, 1000);
    }

    #[test]
    fn max_pow10_no_more_than_test5() {
	let x: u32 = 12345;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 4);
	assert_eq!(max_ten_kappa, 10000);
    }

    #[test]
    fn max_pow10_no_more_than_test6() {
	let x: u32 = 123456;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 5);
	assert_eq!(max_ten_kappa, 100000);
    }

    #[test]
    fn max_pow10_no_more_than_test7() {
	let x: u32 = 1234567;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 6);
	assert_eq!(max_ten_kappa, 1000000);
    }

    #[test]
    fn max_pow10_no_more_than_test8() {
	let x: u32 = 12345678;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 7);
	assert_eq!(max_ten_kappa, 10000000);
    }

    #[test]
    fn max_pow10_no_more_than_test9() {
	let x: u32 = 123456789;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 8);
	assert_eq!(max_ten_kappa, 100000000);
    }

    #[test]
    fn max_pow10_no_more_than_test10() {
	let x: u32 = 1234567890;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 9);
	assert_eq!(max_ten_kappa, 1000000000);
    }

    #[test]
    fn max_pow10_no_more_than_test11() {
	let x: u32 = 12345678901;
	let (max_kappa, max_ten_kappa): (u8, u32) = max_pow10_no_more_than(x);

	assert_eq!(max_kappa, 9);
	assert_eq!(max_ten_kappa, 1000000000);
    }
}
