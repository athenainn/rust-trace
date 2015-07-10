#![feature(core)]
extern crate core;
    // macro_rules! e {
    //     ($e:expr) => { $e }
    // }

    // macro_rules! tuple_impls {
    //     ($(
    //         $Tuple:ident {
    //             $(($idx:tt) -> $T:ident)+
    //         }
    //     )+) => {
    //         $(
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<$($T:Clone),+> Clone for ($($T,)+) {
    //                 fn clone(&self) -> ($($T,)+) {
    //                     ($(e!(self.$idx.clone()),)+)
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<$($T:PartialEq),+> PartialEq for ($($T,)+) {
    //                 #[inline]
    //                 fn eq(&self, other: &($($T,)+)) -> bool {
    //                     e!($(self.$idx == other.$idx)&&+)
    //                 }
    //                 #[inline]
    //                 fn ne(&self, other: &($($T,)+)) -> bool {
    //                     e!($(self.$idx != other.$idx)||+)
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<$($T:Eq),+> Eq for ($($T,)+) {}
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<$($T:PartialOrd + PartialEq),+> PartialOrd for ($($T,)+) {
    //                 #[inline]
    //                 fn partial_cmp(&self, other: &($($T,)+)) -> Option<Ordering> {
    //                     lexical_partial_cmp!($(self.$idx, other.$idx),+)
    //                 }
    //                 #[inline]
    //                 fn lt(&self, other: &($($T,)+)) -> bool {
    //                     lexical_ord!(lt, $(self.$idx, other.$idx),+)
    //                 }
    //                 #[inline]
    //                 fn le(&self, other: &($($T,)+)) -> bool {
    //                     lexical_ord!(le, $(self.$idx, other.$idx),+)
    //                 }
    //                 #[inline]
    //                 fn ge(&self, other: &($($T,)+)) -> bool {
    //                     lexical_ord!(ge, $(self.$idx, other.$idx),+)
    //                 }
    //                 #[inline]
    //                 fn gt(&self, other: &($($T,)+)) -> bool {
    //                     lexical_ord!(gt, $(self.$idx, other.$idx),+)
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<$($T:Ord),+> Ord for ($($T,)+) {
    //                 #[inline]
    //                 fn cmp(&self, other: &($($T,)+)) -> Ordering {
    //                     lexical_cmp!($(self.$idx, other.$idx),+)
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<$($T:Default),+> Default for ($($T,)+) {
    //                 #[stable(feature = "rust1", since = "1.0.0")]
    //                 #[inline]
    //                 fn default() -> ($($T,)+) {
    //                     ($({ let x: $T = Default::default(); x},)+)
    //                 }
    //             }
    //         )+
    //     }
    // }

    // // Constructs an expression that performs a lexical ordering using method $rel.
    // // The values are interleaved, so the macro invocation for
    // // `(a1, a2, a3) < (b1, b2, b3)` would be `lexical_ord!(lt, a1, b1, a2, b2,
    // // a3, b3)` (and similarly for `lexical_cmp`)
    // macro_rules! lexical_ord {
    //     ($rel: ident, $a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
    //         if $a != $b { lexical_ord!($rel, $a, $b) }
    //         else { lexical_ord!($rel, $($rest_a, $rest_b),+) }
    //     };
    //     ($rel: ident, $a:expr, $b:expr) => { ($a) . $rel (& $b) };
    // }

    // macro_rules! lexical_partial_cmp {
    //     ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
    //         match ($a).partial_cmp(&$b) {
    //             Some(Equal) => lexical_partial_cmp!($($rest_a, $rest_b),+),
    //             ordering   => ordering
    //         }
    //     };
    //     ($a:expr, $b:expr) => { ($a).partial_cmp(&$b) };
    // }

    // macro_rules! lexical_cmp {
    //     ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
    //         match ($a).cmp(&$b) {
    //             Equal => lexical_cmp!($($rest_a, $rest_b),+),
    //             ordering   => ordering
    //         }
    //     };
    //     ($a:expr, $b:expr) => { ($a).cmp(&$b) };
    // }

    // tuple_impls! {
    //     Tuple1 {
    //         (0) -> A
    //     }
    //     Tuple2 {
    //         (0) -> A
    //         (1) -> B
    //     }
    //     Tuple3 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //     }
    //     Tuple4 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //     }
    //     Tuple5 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //     }
    //     Tuple6 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //         (5) -> F
    //     }
    //     Tuple7 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //         (5) -> F
    //         (6) -> G
    //     }
    //     Tuple8 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //         (5) -> F
    //         (6) -> G
    //         (7) -> H
    //     }
    //     Tuple9 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //         (5) -> F
    //         (6) -> G
    //         (7) -> H
    //         (8) -> I
    //     }
    //     Tuple10 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //         (5) -> F
    //         (6) -> G
    //         (7) -> H
    //         (8) -> I
    //         (9) -> J
    //     }
    //     Tuple11 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //         (5) -> F
    //         (6) -> G
    //         (7) -> H
    //         (8) -> I
    //         (9) -> J
    //         (10) -> K
    //     }
    //     Tuple12 {
    //         (0) -> A
    //         (1) -> B
    //         (2) -> C
    //         (3) -> D
    //         (4) -> E
    //         (5) -> F
    //         (6) -> G
    //         (7) -> H
    //         (8) -> I
    //         (9) -> J
    //         (10) -> K
    //         (11) -> L
    //     }
    // }

#[cfg(test)]
mod tests {
    macro_rules! eq_test {
	(
	    $($T:ident)+
	) => (
	    {
		let left: ($($T,)+) = ($($T::default(),)+);
		let right: ($($T,)+) = ($($T::default(),)+);
		let result: bool = left.eq(&right);

		assert_eq!(result, true);
	    }
	    {
		let left: ($($T,)+) = ($($T::default(),)+);
		let right: ($($T,)+) = ($($T::default(),)+);
		let result: bool = left == right;

		assert_eq!(result, true);
	    }
	    {
		let left: ($($T,)+) = ($($T::default(),)+);
		let right: ($($T,)+) = ($($T::default() + 1 as $T,)+);
		let result: bool = left.eq(&right);

		assert_eq!(result, false);
	    }
	    {
		let left: ($($T,)+) = ($($T::default(),)+);
		let right: ($($T,)+) = ($($T::default() + 1 as $T,)+);
		let result: bool = left == right;

		assert_eq!(result, false);
	    }
	)
    }

    type A = u8;
    type B = u16;
    type C = u32;
    type D = u64;
    type E = usize;
    type F = i8;
    type G = i16;
    type H = i32;
    type I = i64;
    type J = isize;
    type K = f32;
    type L = f64;

    #[test]
    fn eq_test1() {
	eq_test! { A B C E F G H I J K L };
    }
}
