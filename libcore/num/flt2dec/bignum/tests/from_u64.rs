#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::bignum::tests::Big8x3;

    // macro_rules! define_bignum {
    //     ($name:ident: type=$ty:ty, n=$n:expr) => (
    //         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
    //         ///
    //         /// This is backed by an fixed-size array of given type ("digit").
    //         /// While the array is not very large (normally some hundred bytes),
    //         /// copying it recklessly may result in the performance hit.
    //         /// Thus this is intentionally not `Copy`.
    //         ///
    //         /// All operations available to bignums panic in the case of over/underflows.
    //         /// The caller is responsible to use large enough bignum types.
    //         pub struct $name {
    //             /// One plus the offset to the maximum "digit" in use.
    //             /// This does not decrease, so be aware of the computation order.
    //             /// `base[size..]` should be zero.
    //             size: usize,
    //             /// Digits. `[a, b, c, ...]` represents `a + b*2^W + c*2^(2W) + ...`
    //             /// where `W` is the number of bits in the digit type.
    //             base: [$ty; $n]
    //         }
    //
    //         impl $name {
    //             /// Makes a bignum from one digit.
    //             pub fn from_small(v: $ty) -> $name {
    //                 let mut base = [0; $n];
    //                 base[0] = v;
    //                 $name { size: 1, base: base }
    //             }
    //
    //             /// Makes a bignum from `u64` value.
    //             pub fn from_u64(mut v: u64) -> $name {
    //                 use mem;
    //
    //                 let mut base = [0; $n];
    //                 let mut sz = 0;
    //                 while v > 0 {
    //                     base[sz] = v as $ty;
    //                     v >>= mem::size_of::<$ty>() * 8;
    //                     sz += 1;
    //                 }
    //                 $name { size: sz, base: base }
    //             }
    //
    //             /// Returns true if the bignum is zero.
    //             pub fn is_zero(&self) -> bool {
    //                 self.base[..self.size].iter().all(|&v| v == 0)
    //             }
    //
    //             /// Adds `other` to itself and returns its own mutable reference.
    //             pub fn add<'a>(&'a mut self, other: &$name) -> &'a mut $name {
    //                 use cmp;
    //                 use num::flt2dec::bignum::FullOps;
    //
    //                 let mut sz = cmp::max(self.size, other.size);
    //                 let mut carry = false;
    //                 for (a, b) in self.base[..sz].iter_mut().zip(&other.base[..sz]) {
    //                     let (c, v) = (*a).full_add(*b, carry);
    //                     *a = v;
    //                     carry = c;
    //                 }
    //                 if carry {
    //                     self.base[sz] = 1;
    //                     sz += 1;
    //                 }
    //                 self.size = sz;
    //                 self
    //             }
    //
    //             /// Subtracts `other` from itself and returns its own mutable reference.
    //             pub fn sub<'a>(&'a mut self, other: &$name) -> &'a mut $name {
    //                 use cmp;
    //                 use num::flt2dec::bignum::FullOps;
    //
    //                 let sz = cmp::max(self.size, other.size);
    //                 let mut noborrow = true;
    //                 for (a, b) in self.base[..sz].iter_mut().zip(&other.base[..sz]) {
    //                     let (c, v) = (*a).full_add(!*b, noborrow);
    //                     *a = v;
    //                     noborrow = c;
    //                 }
    //                 assert!(noborrow);
    //                 self.size = sz;
    //                 self
    //             }
    //
    //             /// Multiplies itself by a digit-sized `other` and returns its own
    //             /// mutable reference.
    //             pub fn mul_small<'a>(&'a mut self, other: $ty) -> &'a mut $name {
    //                 use num::flt2dec::bignum::FullOps;
    //
    //                 let mut sz = self.size;
    //                 let mut carry = 0;
    //                 for a in &mut self.base[..sz] {
    //                     let (c, v) = (*a).full_mul(other, carry);
    //                     *a = v;
    //                     carry = c;
    //                 }
    //                 if carry > 0 {
    //                     self.base[sz] = carry;
    //                     sz += 1;
    //                 }
    //                 self.size = sz;
    //                 self
    //             }
    //
    //             /// Multiplies itself by `2^bits` and returns its own mutable reference.
    //             pub fn mul_pow2<'a>(&'a mut self, bits: usize) -> &'a mut $name {
    //                 use mem;
    //
    //                 let digitbits = mem::size_of::<$ty>() * 8;
    //                 let digits = bits / digitbits;
    //                 let bits = bits % digitbits;
    //
    //                 assert!(digits < $n);
    //                 debug_assert!(self.base[$n-digits..].iter().all(|&v| v == 0));
    //                 debug_assert!(bits == 0 || (self.base[$n-digits-1] >> (digitbits - bits)) == 0);
    //
    //                 // shift by `digits * digitbits` bits
    //                 for i in (0..self.size).rev() {
    //                     self.base[i+digits] = self.base[i];
    //                 }
    //                 for i in 0..digits {
    //                     self.base[i] = 0;
    //                 }
    //
    //                 // shift by `bits` bits
    //                 let mut sz = self.size + digits;
    //                 if bits > 0 {
    //                     let last = sz;
    //                     let overflow = self.base[last-1] >> (digitbits - bits);
    //                     if overflow > 0 {
    //                         self.base[last] = overflow;
    //                         sz += 1;
    //                     }
    //                     for i in (digits+1..last).rev() {
    //                         self.base[i] = (self.base[i] << bits) |
    //                                        (self.base[i-1] >> (digitbits - bits));
    //                     }
    //                     self.base[digits] <<= bits;
    //                     // self.base[..digits] is zero, no need to shift
    //                 }
    //
    //                 self.size = sz;
    //                 self
    //             }
    //
    //             /// Multiplies itself by a number described by `other[0] + other[1] * 2^W +
    //             /// other[2] * 2^(2W) + ...` (where `W` is the number of bits in the digit type)
    //             /// and returns its own mutable reference.
    //             pub fn mul_digits<'a>(&'a mut self, other: &[$ty]) -> &'a mut $name {
    //                 // the internal routine. works best when aa.len() <= bb.len().
    //                 fn mul_inner(ret: &mut [$ty; $n], aa: &[$ty], bb: &[$ty]) -> usize {
    //                     use num::flt2dec::bignum::FullOps;
    //
    //                     let mut retsz = 0;
    //                     for (i, &a) in aa.iter().enumerate() {
    //                         if a == 0 { continue; }
    //                         let mut sz = bb.len();
    //                         let mut carry = 0;
    //                         for (j, &b) in bb.iter().enumerate() {
    //                             let (c, v) = a.full_mul_add(b, ret[i + j], carry);
    //                             ret[i + j] = v;
    //                             carry = c;
    //                         }
    //                         if carry > 0 {
    //                             ret[i + sz] = carry;
    //                             sz += 1;
    //                         }
    //                         if retsz < i + sz {
    //                             retsz = i + sz;
    //                         }
    //                     }
    //                     retsz
    //                 }
    //
    //                 let mut ret = [0; $n];
    //                 let retsz = if self.size < other.len() {
    //                     mul_inner(&mut ret, &self.base[..self.size], other)
    //                 } else {
    //                     mul_inner(&mut ret, other, &self.base[..self.size])
    //                 };
    //                 self.base = ret;
    //                 self.size = retsz;
    //                 self
    //             }
    //
    //             /// Divides itself by a digit-sized `other` and returns its own
    //             /// mutable reference *and* the remainder.
    //             pub fn div_rem_small<'a>(&'a mut self, other: $ty) -> (&'a mut $name, $ty) {
    //                 use num::flt2dec::bignum::FullOps;
    //
    //                 assert!(other > 0);
    //
    //                 let sz = self.size;
    //                 let mut borrow = 0;
    //                 for a in self.base[..sz].iter_mut().rev() {
    //                     let (q, r) = (*a).full_div_rem(other, borrow);
    //                     *a = q;
    //                     borrow = r;
    //                 }
    //                 (self, borrow)
    //             }
    //         }
    //
    //         impl ::cmp::PartialEq for $name {
    //             fn eq(&self, other: &$name) -> bool { self.base[..] == other.base[..] }
    //         }
    //
    //         impl ::cmp::Eq for $name {
    //         }
    //
    //         impl ::cmp::PartialOrd for $name {
    //             fn partial_cmp(&self, other: &$name) -> ::option::Option<::cmp::Ordering> {
    //                 ::option::Option::Some(self.cmp(other))
    //             }
    //         }
    //
    //         impl ::cmp::Ord for $name {
    //             fn cmp(&self, other: &$name) -> ::cmp::Ordering {
    //                 use cmp::max;
    //                 use iter::order;
    //
    //                 let sz = max(self.size, other.size);
    //                 let lhs = self.base[..sz].iter().cloned().rev();
    //                 let rhs = other.base[..sz].iter().cloned().rev();
    //                 order::cmp(lhs, rhs)
    //             }
    //         }
    //
    //         impl ::clone::Clone for $name {
    //             fn clone(&self) -> $name {
    //                 $name { size: self.size, base: self.base }
    //             }
    //         }
    //
    //         impl ::fmt::Debug for $name {
    //             fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
    //                 use mem;
    //
    //                 let sz = if self.size < 1 {1} else {self.size};
    //                 let digitlen = mem::size_of::<$ty>() * 2;
    //
    //                 try!(write!(f, "{:#x}", self.base[sz-1]));
    //                 for &v in self.base[..sz-1].iter().rev() {
    //                     try!(write!(f, "_{:01$x}", v, digitlen));
    //                 }
    //                 ::result::Result::Ok(())
    //             }
    //         }
    //     )
    // }

    // define_bignum!(Big8x3: type=u8, n=3);

    #[test]
    fn from_u64_test1() {
	let v: u64 = 0x123456;
	let big: Big8x3 = Big8x3::from_u64(v);
	let result: String = format!("{:?}", big);
	assert_eq!(result, "0x12_34_56".to_string());
    }

    #[test]
    #[should_panic]
    fn from_u64_test2() {
	let v: u64 = 0x12345678;
	let _: Big8x3 = Big8x3::from_u64(v); // panicked at 'index out of bounds: the len is 3 but the index is 3'
    }
}
