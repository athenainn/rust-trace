#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::discriminant_value;

    // pub fn discriminant_value<T>(v: &T) -> u64;

    macro_rules! discriminant_value_test {
	($v:expr, $result:expr) => ({
	    let result: u64 = unsafe { discriminant_value($v) };

	    assert_eq!(result, $result);
	})
    }

    #[test]
    fn discriminant_value_test1() {
	enum CLike {
	    A,
	    B,
	    C,
	    D
	}

	discriminant_value_test!( &CLike::A, 0 );
	discriminant_value_test!( &CLike::B, 1 );
	discriminant_value_test!( &CLike::C, 2 );
	discriminant_value_test!( &CLike::D, 3 );
    }

    #[test]
    fn discriminant_value_test2() {
	enum CLike {
	    A = 5,
	    B = 2,
	    C = 19,
	    D
	}

	discriminant_value_test!( &CLike::A, 5 );
	discriminant_value_test!( &CLike::B, 2 );
	discriminant_value_test!( &CLike::C, 19 );
	discriminant_value_test!( &CLike::D, 20 );
    }

    #[test]
    fn discriminant_value_test3() {
	enum CLike {
	    A = 5,
	    B,
	    C = -1,
	    D
	}

	discriminant_value_test!( &CLike::A, 5 );
	discriminant_value_test!( &CLike::B, 6 );
	discriminant_value_test!( &CLike::C, -1_i8 as u64 );
	discriminant_value_test!( &CLike::D, 0 );
    }

    #[test]
    fn discriminant_value_test4() {
	enum ADT {
	    First(u32, u32),
	    Second(u64)
	}

	discriminant_value_test!( &ADT::First(0, 0), 0 );
	discriminant_value_test!( &ADT::Second(5), 1 );
    }

    #[test]
    fn discriminant_value_test5() {
	static CONST: u32 = 0xbeef;

	enum NullablePointer {
	    Something(&'static u32),
	    Nothing
	}

	discriminant_value_test!( &NullablePointer::Nothing, 1 );
	discriminant_value_test!( &NullablePointer::Something(&CONST), 0 );
    }

    #[test]
    fn discriminant_value_test6() {
	discriminant_value_test!( &10, 0 );
	discriminant_value_test!( &"test", 0 );
    }
}
