#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::clone::Clone;

    // pub trait FixedSizeArray<T> {
    //     /// Converts the array to immutable slice
    //     fn as_slice(&self) -> &[T];
    //     /// Converts the array to mutable slice
    //     fn as_mut_slice(&mut self) -> &mut [T];
    // }

    // macro_rules! array_impls {
    //     ($($N:expr)+) => {
    //         $(
    //             #[unstable(feature = "core")]
    //             impl<T> FixedSizeArray<T> for [T; $N] {
    //                 #[inline]
    //                 fn as_slice(&self) -> &[T] {
    //                     &self[..]
    //                 }
    //                 #[inline]
    //                 fn as_mut_slice(&mut self) -> &mut [T] {
    //                     &mut self[..]
    //                 }
    //             }
    //
    //             #[unstable(feature = "array_as_ref",
    //                        reason = "should ideally be implemented for all fixed-sized arrays")]
    //             impl<T> AsRef<[T]> for [T; $N] {
    //                 #[inline]
    //                 fn as_ref(&self) -> &[T] {
    //                     &self[..]
    //                 }
    //             }
    //
    //             #[unstable(feature = "array_as_ref",
    //                        reason = "should ideally be implemented for all fixed-sized arrays")]
    //             impl<T> AsMut<[T]> for [T; $N] {
    //                 #[inline]
    //                 fn as_mut(&mut self) -> &mut [T] {
    //                     &mut self[..]
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<T:Copy> Clone for [T; $N] {
    //                 fn clone(&self) -> [T; $N] {
    //                     *self
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<T: Hash> Hash for [T; $N] {
    //                 fn hash<H: hash::Hasher>(&self, state: &mut H) {
    //                     Hash::hash(&self[..], state)
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<T: fmt::Debug> fmt::Debug for [T; $N] {
    //                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //                     fmt::Debug::fmt(&&self[..], f)
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<'a, T> IntoIterator for &'a [T; $N] {
    //                 type Item = &'a T;
    //                 type IntoIter = Iter<'a, T>;
    // 
    //                 fn into_iter(self) -> Iter<'a, T> {
    //                     self.iter()
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<'a, T> IntoIterator for &'a mut [T; $N] {
    //                 type Item = &'a mut T;
    //                 type IntoIter = IterMut<'a, T>;
    //
    //                 fn into_iter(self) -> IterMut<'a, T> {
    //                     self.iter_mut()
    //                 }
    //             }
    //
    //             // NOTE: some less important impls are omitted to reduce code bloat
    //             __impl_slice_eq1! { [A; $N], [B; $N] }
    //             __impl_slice_eq2! { [A; $N], [B] }
    //             __impl_slice_eq2! { [A; $N], &'b [B] }
    //             __impl_slice_eq2! { [A; $N], &'b mut [B] }
    //             // __impl_slice_eq2! { [A; $N], &'b [B; $N] }
    //             // __impl_slice_eq2! { [A; $N], &'b mut [B; $N] }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<T:Eq> Eq for [T; $N] { }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<T:PartialOrd> PartialOrd for [T; $N] {
    //                 #[inline]
    //                 fn partial_cmp(&self, other: &[T; $N]) -> Option<Ordering> {
    //                     PartialOrd::partial_cmp(&&self[..], &&other[..])
    //                 }
    //                 #[inline]
    //                 fn lt(&self, other: &[T; $N]) -> bool {
    //                     PartialOrd::lt(&&self[..], &&other[..])
    //                 }
    //                 #[inline]
    //                 fn le(&self, other: &[T; $N]) -> bool {
    //                     PartialOrd::le(&&self[..], &&other[..])
    //                 }
    //                 #[inline]
    //                 fn ge(&self, other: &[T; $N]) -> bool {
    //                     PartialOrd::ge(&&self[..], &&other[..])
    //                 }
    //                 #[inline]
    //                 fn gt(&self, other: &[T; $N]) -> bool {
    //                     PartialOrd::gt(&&self[..], &&other[..])
    //                 }
    //             }
    //
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<T:Ord> Ord for [T; $N] {
    //                 #[inline]
    //                 fn cmp(&self, other: &[T; $N]) -> Ordering {
    //                     Ord::cmp(&&self[..], &&other[..])
    //                 }
    //             }
    //         )+
    //     }
    // }

    // array_impls! {
    //      0  1  2  3  4  5  6  7  8  9
    //     10 11 12 13 14 15 16 17 18 19
    //     20 21 22 23 24 25 26 27 28 29
    //     30 31 32
    // }

    type T = i32;

    #[test]
    fn clone_test1() {
	let array: [T; 26] = [
	     0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
	    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
	    20, 21, 22, 23, 24, 25
	];
	let clone: [T; 26] = array.clone();

	for i in 0..26 {
	    assert_eq!(array[i], clone[i]);
	}
    }
}
