#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::hash::Hash;
    use core::hash::Hasher;
    use core::hash::SipHasher;

    // pub struct SipHasher {
    //     k0: u64,
    //     k1: u64,
    //     length: usize, // how many bytes we've processed
    //     v0: u64,      // hash state
    //     v1: u64,
    //     v2: u64,
    //     v3: u64,
    //     tail: u64, // unprocessed bytes le
    //     ntail: usize,  // how many bytes in tail are valid
    // }

    // macro_rules! u8to64_le {
    //     ($buf:expr, $i:expr) =>
    //     ($buf[0+$i] as u64 |
    //      ($buf[1+$i] as u64) << 8 |
    //      ($buf[2+$i] as u64) << 16 |
    //      ($buf[3+$i] as u64) << 24 |
    //      ($buf[4+$i] as u64) << 32 |
    //      ($buf[5+$i] as u64) << 40 |
    //      ($buf[6+$i] as u64) << 48 |
    //      ($buf[7+$i] as u64) << 56);
    //     ($buf:expr, $i:expr, $len:expr) =>
    //     ({
    //         let mut t = 0;
    //         let mut out = 0;
    //         while t < $len {
    //             out |= ($buf[t+$i] as u64) << t*8;
    //             t += 1;
    //         }
    //         out
    //     });
    // }

    // macro_rules! rotl {
    //     ($x:expr, $b:expr) =>
    //     (($x << $b) | ($x >> (64_i32.wrapping_sub($b))))
    // }

    // macro_rules! compress {
    //     ($v0:expr, $v1:expr, $v2:expr, $v3:expr) =>
    //     ({
    //         $v0 = $v0.wrapping_add($v1); $v1 = rotl!($v1, 13); $v1 ^= $v0;
    //         $v0 = rotl!($v0, 32);
    //         $v2 = $v2.wrapping_add($v3); $v3 = rotl!($v3, 16); $v3 ^= $v2;
    //         $v0 = $v0.wrapping_add($v3); $v3 = rotl!($v3, 21); $v3 ^= $v0;
    //         $v2 = $v2.wrapping_add($v1); $v1 = rotl!($v1, 17); $v1 ^= $v2;
    //         $v2 = rotl!($v2, 32);
    //     })
    // }

    // impl SipHasher {
    //     /// Creates a new `SipHasher` with the two initial keys set to 0.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new() -> SipHasher {
    //         SipHasher::new_with_keys(0, 0)
    //     }
    //
    //     /// Creates a `SipHasher` that is keyed off the provided keys.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new_with_keys(key0: u64, key1: u64) -> SipHasher {
    //         let mut state = SipHasher {
    //             k0: key0,
    //             k1: key1,
    //             length: 0,
    //             v0: 0,
    //             v1: 0,
    //             v2: 0,
    //             v3: 0,
    //             tail: 0,
    //             ntail: 0,
    //         };
    //         state.reset();
    //         state
    //     }
    //
    //     #[inline]
    //     fn reset(&mut self) {
    //         self.length = 0;
    //         self.v0 = self.k0 ^ 0x736f6d6570736575;
    //         self.v1 = self.k1 ^ 0x646f72616e646f6d;
    //         self.v2 = self.k0 ^ 0x6c7967656e657261;
    //         self.v3 = self.k1 ^ 0x7465646279746573;
    //         self.ntail = 0;
    //     }
    //
    //     #[inline]
    //     fn write(&mut self, msg: &[u8]) {
    //         let length = msg.len();
    //         self.length += length;
    //
    //         let mut needed = 0;
    //
    //         if self.ntail != 0 {
    //             needed = 8 - self.ntail;
    //             if length < needed {
    //                 self.tail |= u8to64_le!(msg, 0, length) << 8*self.ntail;
    //                 self.ntail += length;
    //                 return
    //             }
    //
    //             let m = self.tail | u8to64_le!(msg, 0, needed) << 8*self.ntail;
    //
    //             self.v3 ^= m;
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             self.v0 ^= m;
    //
    //             self.ntail = 0;
    //         }
    //
    //         // Buffered tail is now flushed, process new input.
    //         let len = length - needed;
    //         let end = len & (!0x7);
    //         let left = len & 0x7;
    //
    //         let mut i = needed;
    //         while i < end {
    //             let mi = u8to64_le!(msg, i);
    //
    //             self.v3 ^= mi;
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             self.v0 ^= mi;
    //
    //             i += 8;
    //         }
    //
    //         self.tail = u8to64_le!(msg, i, left);
    //         self.ntail = left;
    //     }
    // }

    // impl Hasher for SipHasher {
    //     #[inline]
    //     fn write(&mut self, msg: &[u8]) {
    //         self.write(msg)
    //     }
    //
    //     #[inline]
    //     fn finish(&self) -> u64 {
    //         let mut v0 = self.v0;
    //         let mut v1 = self.v1;
    //         let mut v2 = self.v2;
    //         let mut v3 = self.v3;
    //
    //         let b: u64 = ((self.length as u64 & 0xff) << 56) | self.tail;
    //
    //         v3 ^= b;
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //         v0 ^= b;
    //
    //         v2 ^= 0xff;
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //
    //         v0 ^ v1 ^ v2 ^ v3
    //     }
    // }

    // pub trait Hash {
    //     /// Feeds this value into the state given, updating the hasher as necessary.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn hash<H: Hasher>(&self, state: &mut H);
    //
    //     /// Feeds a slice of this type into the state provided.
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) where Self: Sized {
    //         for piece in data {
    //             piece.hash(state);
    //         }
    //     }
    // }

    // pub trait Hasher {
    //     /// Completes a round of hashing, producing the output hash generated.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn finish(&self) -> u64;
    //
    //     /// Writes some data into this `Hasher`
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn write(&mut self, bytes: &[u8]);
    //
    //     /// Write a single `u8` into this hasher
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_u8(&mut self, i: u8) { self.write(&[i]) }
    //     /// Write a single `u16` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_u16(&mut self, i: u16) {
    //         self.write(&unsafe { mem::transmute::<_, [u8; 2]>(i) })
    //     }
    //     /// Write a single `u32` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_u32(&mut self, i: u32) {
    //         self.write(&unsafe { mem::transmute::<_, [u8; 4]>(i) })
    //     }
    //     /// Write a single `u64` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_u64(&mut self, i: u64) {
    //         self.write(&unsafe { mem::transmute::<_, [u8; 8]>(i) })
    //     }
    //     /// Write a single `usize` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_usize(&mut self, i: usize) {
    //         if cfg!(target_pointer_width = "32") {
    //             self.write_u32(i as u32)
    //         } else {
    //             self.write_u64(i as u64)
    //         }
    //     }
    //
    //     /// Write a single `i8` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_i8(&mut self, i: i8) { self.write_u8(i as u8) }
    //     /// Write a single `i16` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_i16(&mut self, i: i16) { self.write_u16(i as u16) }
    //     /// Write a single `i32` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_i32(&mut self, i: i32) { self.write_u32(i as u32) }
    //     /// Write a single `i64` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_i64(&mut self, i: i64) { self.write_u64(i as u64) }
    //     /// Write a single `isize` into this hasher.
    //     #[inline]
    //     #[unstable(feature = "hash", reason = "module was recently redesigned")]
    //     fn write_isize(&mut self, i: isize) { self.write_usize(i as usize) }
    // }

    //     macro_rules! impl_write {
    //         ($(($ty:ident, $meth:ident),)*) => {$(
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl Hash for $ty {
    //                 fn hash<H: Hasher>(&self, state: &mut H) {
    //                     state.$meth(*self)
    //                 }
    //
    //                 fn hash_slice<H: Hasher>(data: &[$ty], state: &mut H) {
    //                     // FIXME(#23542) Replace with type ascription.
    //                     #![allow(trivial_casts)]
    //                     let newlen = data.len() * ::$ty::BYTES;
    //                     let ptr = data.as_ptr() as *const u8;
    //                     state.write(unsafe { slice::from_raw_parts(ptr, newlen) })
    //                 }
    //             }
    //         )*}
    //     }

    //     impl_write! {
    //         (u8, write_u8),
    //         (u16, write_u16),
    //         (u32, write_u32),
    //         (u64, write_u64),
    //         (usize, write_usize),
    //         (i8, write_i8),
    //         (i16, write_i16),
    //         (i32, write_i32),
    //         (i64, write_i64),
    //         (isize, write_isize),
    //     }

    type H = SipHasher; // H: Hasher

    #[test]
    fn hash_test1() {
	let mut state: H = <H>::new();
	let finish: u64 = state.finish();
	assert_eq!(finish, 0x1e924b9d737700d7);

	let data: i64 = 0x0102030405060708;
	data.hash::<H>(&mut state);

	let finish: u64 = state.finish();
	assert_eq!(finish, 0x6e30be5baa0e07fe);
    }
}
