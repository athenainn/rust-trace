#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::next_code_point;

    use core::slice;

    // pub fn next_code_point(bytes: &mut slice::Iter<u8>) -> Option<u32> {
    //     // Decode UTF-8
    //     let x = match bytes.next() {
    //         None => return None,
    //         Some(&next_byte) if next_byte < 128 => return Some(next_byte as u32),
    //         Some(&next_byte) => next_byte,
    //     };
    //
    //     // Multibyte case follows
    //     // Decode from a byte combination out of: [[[x y] z] w]
    //     // NOTE: Performance is sensitive to the exact formulation here
    //     let init = utf8_first_byte(x, 2);
    //     let y = unwrap_or_0(bytes.next());
    //     let mut ch = utf8_acc_cont_byte(init, y);
    //     if x >= 0xE0 {
    //         // [[x y z] w] case
    //         // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
    //         let z = unwrap_or_0(bytes.next());
    //         let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
    //         ch = init << 12 | y_z;
    //         if x >= 0xF0 {
    //             // [x y z w] case
    //             // use only the lower 3 bits of `init`
    //             let w = unwrap_or_0(bytes.next());
    //             ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
    //         }
    //     }
    //
    //     Some(ch)
    // }

    #[test]
    fn next_code_point_test1() {
	let bytes: &[u8] = &[];
	let mut iter: slice::Iter<u8> = bytes.iter();
	let result: Option<u32> = next_code_point(&mut iter);

	match result {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}
    }

    #[test]
    fn next_code_point_test2() {
	let bytes: &[u8] = &[0b0111_1111];
	let mut iter: slice::Iter<u8> = bytes.iter();
	let next_code_point: Option<u32> = next_code_point(&mut iter);
	let result: u32 = next_code_point.unwrap();

	assert_eq!(result, 0b0111_1111);
    }

    #[test]
    fn next_code_point_test3() {
	let bytes: &[u8] = &[0b1101_1111, 0b1011_1111];
	let mut iter: slice::Iter<u8> = bytes.iter();
	let next_code_point: Option<u32> = next_code_point(&mut iter);
	let result: u32 = next_code_point.unwrap();

	assert_eq!(result, 0b0111_1111_1111);
    }

    #[test]
    fn next_code_point_test4() {
	let bytes: &[u8] = &[0b1110_1111, 0b1011_1111, 0b1011_1111];
	let mut iter: slice::Iter<u8> = bytes.iter();
	let next_code_point: Option<u32> = next_code_point(&mut iter);
	let result: u32 = next_code_point.unwrap();

	assert_eq!(result, 0b1111_1111_1111_1111);
    }

    #[test]
    fn next_code_point_test5() {
	let bytes: &[u8] = &[0b1111_0111, 0b1011_1111, 0b1011_1111, 0b1011_1111];
	let mut iter: slice::Iter<u8> = bytes.iter();
	let next_code_point: Option<u32> = next_code_point(&mut iter);
	let result: u32 = next_code_point.unwrap();

	assert_eq!(result, 0b0001_1111_1111_1111_1111_1111);
    }
}
