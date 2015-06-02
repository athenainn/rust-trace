#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::Iter;

    // impl<'a, T> Iter<'a, T> {
    //     /// View the underlying data as a subslice of the original data.
    //     ///
    //     /// This has the same lifetime as the original slice, and so the
    //     /// iterator can continue to be used while this exists.
    //     #[unstable(feature = "core")]
    //     pub fn as_slice(&self) -> &'a [T] {
    //         make_slice!(self.ptr, self.end)
    //     }
    //
    //     // Helper function for Iter::nth
    //     fn iter_nth(&mut self, n: usize) -> Option<&'a T> {
    //         match self.as_slice().get(n) {
    //             Some(elem_ref) => unsafe {
    //                 self.ptr = slice_offset!(self.ptr, (n as isize).wrapping_add(1));
    //                 Some(elem_ref)
    //             },
    //             None => {
    //                 self.ptr = self.end;
    //                 None
    //             }
    //         }
    //     }
    // }

    // fn size_from_ptr<T>(_: *const T) -> usize {
    //     mem::size_of::<T>()
    // }

    // macro_rules! slice_offset {
    //     ($ptr:expr, $by:expr) => {{
    //         let ptr = $ptr;
    //         if size_from_ptr(ptr) == 0 {
    //             ::intrinsics::arith_offset(ptr as *mut i8, $by) as *mut _
    //         } else {
    //             ptr.offset($by)
    //         }
    //     }};
    // }

    // macro_rules! slice_ref {
    //     ($ptr:expr) => {{
    //         let ptr = $ptr;
    //         if size_from_ptr(ptr) == 0 {
    //             // Use a non-null pointer value
    //             &mut *(1 as *mut _)
    //         } else {
    //             transmute(ptr)
    //         }
    //     }};
    // }

    // pub unsafe fn from_raw_parts<'a, T>(p: *const T, len: usize) -> &'a [T] {
    //     transmute(RawSlice { data: p, len: len })
    // }

    // macro_rules! make_slice {
    //     ($start: expr, $end: expr) => {{
    //         let start = $start;
    //         let diff = ($end as usize).wrapping_sub(start as usize);
    //         if size_from_ptr(start) == 0 {
    //             // use a non-null pointer value
    //             unsafe { from_raw_parts(1 as *const _, diff) }
    //         } else {
    //             let len = diff / size_from_ptr(start);
    //             unsafe { from_raw_parts(start, len) }
    //         }
    //     }}
    // }

    // impl<'a, T> IntoIterator for &'a [T] {
    //     type Item = &'a T;
    //     type IntoIter = Iter<'a, T>;
    //
    //     fn into_iter(self) -> Iter<'a, T> {
    //         self.iter()
    //     }
    // }

    // macro_rules! iterator {
    //     (struct $name:ident -> $ptr:ty, $elem:ty) => {
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl<'a, T> Iterator for $name<'a, T> {
    //             type Item = $elem;
    //
    //             #[inline]
    //             fn next(&mut self) -> Option<$elem> {
    //                 // could be implemented with slices, but this avoids bounds checks
    //                 unsafe {
    //                     if mem::size_of::<T>() != 0 {
    //                         assume(!self.ptr.is_null());
    //                         assume(!self.end.is_null());
    //                     }
    //                     if self.ptr == self.end {
    //                         None
    //                     } else {
    //                         let old = self.ptr;
    //                         self.ptr = slice_offset!(self.ptr, 1);
    //                         Some(slice_ref!(old))
    //                     }
    //                 }
    //             }
    //
    //             #[inline]
    //             fn size_hint(&self) -> (usize, Option<usize>) {
    //                 let diff = (self.end as usize).wrapping_sub(self.ptr as usize);
    //                 let size = mem::size_of::<T>();
    //                 let exact = diff / (if size == 0 {1} else {size});
    //                 (exact, Some(exact))
    //             }
    //
    //             #[inline]
    //             fn count(self) -> usize {
    //                 self.size_hint().0
    //             }
    //
    //             #[inline]
    //             fn nth(&mut self, n: usize) -> Option<$elem> {
    //                 // Call helper method. Can't put the definition here because mut versus const.
    //                 self.iter_nth(n)
    //             }
    //
    //             #[inline]
    //             fn last(mut self) -> Option<$elem> {
    //                 self.next_back()
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl<'a, T> DoubleEndedIterator for $name<'a, T> {
    //             #[inline]
    //             fn next_back(&mut self) -> Option<$elem> {
    //                 // could be implemented with slices, but this avoids bounds checks
    //                 unsafe {
    //                     if mem::size_of::<T>() != 0 {
    //                         assume(!self.ptr.is_null());
    //                         assume(!self.end.is_null());
    //                     }
    //                     if self.end == self.ptr {
    //                         None
    //                     } else {
    //                         self.end = slice_offset!(self.end, -1);
    //                         Some(slice_ref!(self.end))
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // iterator!{struct Iter -> *const T, &'a T}

    type T = i32;

    #[test]
    fn into_iter_test1() {
	let slice: &[T] = &[1, 2, 3, 4, 5, 6];
	let mut into_iter: Iter<T> = slice.into_iter();
	let len: usize = into_iter.size_hint().0;

	for i in 0..len {
	    let x: Option<&T> = into_iter.next();
	    match x {
		Some(v) => assert_eq!(*v, (i + 1) as T),
		None => assert!(false)
	    }
	}
    }
}
