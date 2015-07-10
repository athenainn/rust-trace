#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::SplitTerminator;

    use core::ops::FnOnce;
    use core::ops::FnMut;

    // impl StrExt for str {
    //     #[inline]
    //     fn contains<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
    //         pat.is_contained_in(self)
    //     }
    //
    //     #[inline]
    //     fn contains_char<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
    //         pat.is_contained_in(self)
    //     }
    //
    //     #[inline]
    //     fn chars(&self) -> Chars {
    //         Chars{iter: self.as_bytes().iter()}
    //     }
    //
    //     #[inline]
    //     fn bytes(&self) -> Bytes {
    //         Bytes(self.as_bytes().iter().map(BytesDeref))
    //     }
    //
    //     #[inline]
    //     fn char_indices(&self) -> CharIndices {
    //         CharIndices { front_offset: 0, iter: self.chars() }
    //     }
    //
    //     #[inline]
    //     fn split<'a, P: Pattern<'a>>(&'a self, pat: P) -> Split<'a, P> {
    //         Split(SplitInternal {
    //             start: 0,
    //             end: self.len(),
    //             matcher: pat.into_searcher(self),
    //             allow_trailing_empty: true,
    //             finished: false,
    //         })
    //     }
    //
    //     #[inline]
    //     fn rsplit<'a, P: Pattern<'a>>(&'a self, pat: P) -> RSplit<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RSplit(self.split(pat).0)
    //     }
    //
    //     #[inline]
    //     fn splitn<'a, P: Pattern<'a>>(&'a self, count: usize, pat: P) -> SplitN<'a, P> {
    //         SplitN(SplitNInternal {
    //             iter: self.split(pat).0,
    //             count: count,
    //         })
    //     }
    //
    //     #[inline]
    //     fn rsplitn<'a, P: Pattern<'a>>(&'a self, count: usize, pat: P) -> RSplitN<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RSplitN(self.splitn(count, pat).0)
    //     }
    //
    //     #[inline]
    //     fn split_terminator<'a, P: Pattern<'a>>(&'a self, pat: P) -> SplitTerminator<'a, P> {
    //         SplitTerminator(SplitInternal {
    //             allow_trailing_empty: false,
    //             ..self.split(pat).0
    //         })
    //     }
    //
    //     #[inline]
    //     fn rsplit_terminator<'a, P: Pattern<'a>>(&'a self, pat: P) -> RSplitTerminator<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RSplitTerminator(self.split_terminator(pat).0)
    //     }
    //
    //     #[inline]
    //     fn matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> Matches<'a, P> {
    //         Matches(MatchesInternal(pat.into_searcher(self)))
    //     }
    //
    //     #[inline]
    //     fn rmatches<'a, P: Pattern<'a>>(&'a self, pat: P) -> RMatches<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RMatches(self.matches(pat).0)
    //     }
    //
    //     #[inline]
    //     fn match_indices<'a, P: Pattern<'a>>(&'a self, pat: P) -> MatchIndices<'a, P> {
    //         MatchIndices(MatchIndicesInternal(pat.into_searcher(self)))
    //     }
    //
    //     #[inline]
    //     fn rmatch_indices<'a, P: Pattern<'a>>(&'a self, pat: P) -> RMatchIndices<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RMatchIndices(self.match_indices(pat).0)
    //     }
    //     #[inline]
    //     fn lines(&self) -> Lines {
    //         Lines(self.split_terminator('\n'))
    //     }
    //
    //     #[inline]
    //     fn lines_any(&self) -> LinesAny {
    //         LinesAny(self.lines().map(LinesAnyMap))
    //     }
    //
    //     #[inline]
    //     fn char_len(&self) -> usize { self.chars().count() }
    //
    //     fn slice_chars(&self, begin: usize, end: usize) -> &str {
    //         assert!(begin <= end);
    //         let mut count = 0;
    //         let mut begin_byte = None;
    //         let mut end_byte = None;
    //
    //         // This could be even more efficient by not decoding,
    //         // only finding the char boundaries
    //         for (idx, _) in self.char_indices() {
    //             if count == begin { begin_byte = Some(idx); }
    //             if count == end { end_byte = Some(idx); break; }
    //             count += 1;
    //         }
    //         if begin_byte.is_none() && count == begin { begin_byte = Some(self.len()) }
    //         if end_byte.is_none() && count == end { end_byte = Some(self.len()) }
    //
    //         match (begin_byte, end_byte) {
    //             (None, _) => panic!("slice_chars: `begin` is beyond end of string"),
    //             (_, None) => panic!("slice_chars: `end` is beyond end of string"),
    //             (Some(a), Some(b)) => unsafe { self.slice_unchecked(a, b) }
    //         }
    //     }
    //
    //     #[inline]
    //     unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str {
    //         mem::transmute(Slice {
    //             data: self.as_ptr().offset(begin as isize),
    //             len: end - begin,
    //         })
    //     }
    //
    //     #[inline]
    //     fn starts_with<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
    //         pat.is_prefix_of(self)
    //     }
    //
    //     #[inline]
    //     fn ends_with<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         pat.is_suffix_of(self)
    //     }
    //
    //     #[inline]
    //     fn trim_matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> &'a str
    //         where P::Searcher: DoubleEndedSearcher<'a>
    //     {
    //         let mut i = 0;
    //         let mut j = 0;
    //         let mut matcher = pat.into_searcher(self);
    //         if let Some((a, b)) = matcher.next_reject() {
    //             i = a;
    //             j = b; // Rember earliest known match, correct it below if
    //                    // last match is different
    //         }
    //         if let Some((_, b)) = matcher.next_reject_back() {
    //             j = b;
    //         }
    //         unsafe {
    //             // Searcher is known to return valid indices
    //             self.slice_unchecked(i, j)
    //         }
    //     }
    //
    //     #[inline]
    //     fn trim_left_matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> &'a str {
    //         let mut i = self.len();
    //         let mut matcher = pat.into_searcher(self);
    //         if let Some((a, _)) = matcher.next_reject() {
    //             i = a;
    //         }
    //         unsafe {
    //             // Searcher is known to return valid indices
    //             self.slice_unchecked(i, self.len())
    //         }
    //     }
    //
    //     #[inline]
    //     fn trim_right_matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> &'a str
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         let mut j = 0;
    //         let mut matcher = pat.into_searcher(self);
    //         if let Some((_, b)) = matcher.next_reject_back() {
    //             j = b;
    //         }
    //         unsafe {
    //             // Searcher is known to return valid indices
    //             self.slice_unchecked(0, j)
    //         }
    //     }
    //
    //     #[inline]
    //     fn is_char_boundary(&self, index: usize) -> bool {
    //         if index == self.len() { return true; }
    //         match self.as_bytes().get(index) {
    //             None => false,
    //             Some(&b) => b < 128 || b >= 192,
    //         }
    //     }
    //
    //     #[inline]
    //     fn char_range_at(&self, i: usize) -> CharRange {
    //         let (c, n) = char_range_at_raw(self.as_bytes(), i);
    //         CharRange { ch: unsafe { mem::transmute(c) }, next: n }
    //     }
    //
    //     #[inline]
    //     fn char_range_at_reverse(&self, start: usize) -> CharRange {
    //         let mut prev = start;
    //
    //         prev = prev.saturating_sub(1);
    //         if self.as_bytes()[prev] < 128 {
    //             return CharRange{ch: self.as_bytes()[prev] as char, next: prev}
    //         }
    //
    //         // Multibyte case is a fn to allow char_range_at_reverse to inline cleanly
    //         fn multibyte_char_range_at_reverse(s: &str, mut i: usize) -> CharRange {
    //             // while there is a previous byte == 10......
    //             while i > 0 && s.as_bytes()[i] & !CONT_MASK == TAG_CONT_U8 {
    //                 i -= 1;
    //             }
    //
    //             let first= s.as_bytes()[i];
    //             let w = UTF8_CHAR_WIDTH[first as usize];
    //             assert!(w != 0);
    //
    //             let mut val = utf8_first_byte(first, w as u32);
    //             val = utf8_acc_cont_byte(val, s.as_bytes()[i + 1]);
    //             if w > 2 { val = utf8_acc_cont_byte(val, s.as_bytes()[i + 2]); }
    //             if w > 3 { val = utf8_acc_cont_byte(val, s.as_bytes()[i + 3]); }
    //
    //             return CharRange {ch: unsafe { mem::transmute(val) }, next: i};
    //         }
    //
    //         return multibyte_char_range_at_reverse(self, prev);
    //     }
    //
    //     #[inline]
    //     fn char_at(&self, i: usize) -> char {
    //         self.char_range_at(i).ch
    //     }
    //
    //     #[inline]
    //     fn char_at_reverse(&self, i: usize) -> char {
    //         self.char_range_at_reverse(i).ch
    //     }
    //
    //     #[inline]
    //     fn as_bytes(&self) -> &[u8] {
    //         unsafe { mem::transmute(self) }
    //     }
    //
    //     fn find<'a, P: Pattern<'a>>(&'a self, pat: P) -> Option<usize> {
    //         pat.into_searcher(self).next_match().map(|(i, _)| i)
    //     }
    //
    //     fn rfind<'a, P: Pattern<'a>>(&'a self, pat: P) -> Option<usize>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         pat.into_searcher(self).next_match_back().map(|(i, _)| i)
    //     }
    //
    //     fn find_str<'a, P: Pattern<'a>>(&'a self, pat: P) -> Option<usize> {
    //         self.find(pat)
    //     }
    //
    //     #[inline]
    //     fn slice_shift_char(&self) -> Option<(char, &str)> {
    //         if self.is_empty() {
    //             None
    //         } else {
    //             let ch = self.char_at(0);
    //             let next_s = unsafe { self.slice_unchecked(ch.len_utf8(), self.len()) };
    //             Some((ch, next_s))
    //         }
    //     }
    //
    //     fn subslice_offset(&self, inner: &str) -> usize {
    //         let a_start = self.as_ptr() as usize;
    //         let a_end = a_start + self.len();
    //         let b_start = inner.as_ptr() as usize;
    //         let b_end = b_start + inner.len();
    //
    //         assert!(a_start <= b_start);
    //         assert!(b_end <= a_end);
    //         b_start - a_start
    //     }
    //
    //     #[inline]
    //     fn as_ptr(&self) -> *const u8 {
    //         self.repr().data
    //     }
    //
    //     #[inline]
    //     fn len(&self) -> usize { self.repr().len }
    //
    //     #[inline]
    //     fn is_empty(&self) -> bool { self.len() == 0 }
    //
    //     #[inline]
    //     fn parse<T: FromStr>(&self) -> Result<T, T::Err> { FromStr::from_str(self) }
    // }

    // acro_rules! generate_pattern_iterators {
    //     {
    //         // Forward iterator
    //         forward:
    //             $(#[$forward_iterator_attribute:meta])*
    //             struct $forward_iterator:ident;
    //
    //         // Reverse iterator
    //         reverse:
    //             $(#[$reverse_iterator_attribute:meta])*
    //             struct $reverse_iterator:ident;
    //
    //         // Stability of all generated items
    //         stability:
    //             $(#[$common_stability_attribute:meta])*
    //
    //         // Internal almost-iterator that is being delegated to
    //         internal:
    //             $internal_iterator:ident yielding ($iterty:ty);
    //
    //         // Kind of delgation - either single ended or double ended
    //         delegate $($t:tt)*
    //     } => {
    //         $(#[$forward_iterator_attribute])*
    //         $(#[$common_stability_attribute])*
    //         pub struct $forward_iterator<'a, P: Pattern<'a>>($internal_iterator<'a, P>);
    //
    //         $(#[$common_stability_attribute])*
    //         impl<'a, P: Pattern<'a>> Iterator for $forward_iterator<'a, P> {
    //             type Item = $iterty;
    //
    //             #[inline]
    //             fn next(&mut self) -> Option<$iterty> {
    //                 self.0.next()
    //             }
    //         }
    //
    //         $(#[$common_stability_attribute])*
    //         impl<'a, P: Pattern<'a>> Clone for $forward_iterator<'a, P>
    //             where P::Searcher: Clone
    //         {
    //             fn clone(&self) -> Self {
    //                 $forward_iterator(self.0.clone())
    //             }
    //         }
    //
    //         $(#[$reverse_iterator_attribute])*
    //         $(#[$common_stability_attribute])*
    //         pub struct $reverse_iterator<'a, P: Pattern<'a>>($internal_iterator<'a, P>);
    //
    //         $(#[$common_stability_attribute])*
    //         impl<'a, P: Pattern<'a>> Iterator for $reverse_iterator<'a, P>
    //             where P::Searcher: ReverseSearcher<'a>
    //         {
    //             type Item = $iterty;
    //
    //             #[inline]
    //             fn next(&mut self) -> Option<$iterty> {
    //                 self.0.next_back()
    //             }
    //         }
    //
    //         $(#[$common_stability_attribute])*
    //         impl<'a, P: Pattern<'a>> Clone for $reverse_iterator<'a, P>
    //             where P::Searcher: Clone
    //         {
    //             fn clone(&self) -> Self {
    //                 $reverse_iterator(self.0.clone())
    //             }
    //         }
    //
    //         generate_pattern_iterators!($($t)* with $(#[$common_stability_attribute])*,
    //                                                 $forward_iterator,
    //                                                 $reverse_iterator, $iterty);
    //     };
    //     {
    //         double ended; with $(#[$common_stability_attribute:meta])*,
    //                            $forward_iterator:ident,
    //                            $reverse_iterator:ident, $iterty:ty
    //     } => {
    //         $(#[$common_stability_attribute])*
    //         impl<'a, P: Pattern<'a>> DoubleEndedIterator for $forward_iterator<'a, P>
    //             where P::Searcher: DoubleEndedSearcher<'a>
    //         {
    //             #[inline]
    //             fn next_back(&mut self) -> Option<$iterty> {
    //                 self.0.next_back()
    //             }
    //         }
    //
    //         $(#[$common_stability_attribute])*
    //         impl<'a, P: Pattern<'a>> DoubleEndedIterator for $reverse_iterator<'a, P>
    //             where P::Searcher: DoubleEndedSearcher<'a>
    //         {
    //             #[inline]
    //             fn next_back(&mut self) -> Option<$iterty> {
    //                 self.0.next()
    //             }
    //         }
    //     };
    //     {
    //         single ended; with $(#[$common_stability_attribute:meta])*,
    //                            $forward_iterator:ident,
    //                            $reverse_iterator:ident, $iterty:ty
    //     } => {}
    // }

    // generate_pattern_iterators! {
    //     forward:
    //         /// Created with the method `.split_terminator()`.
    //         struct SplitTerminator;
    //     reverse:
    //         /// Created with the method `.rsplit_terminator()`.
    //         struct RSplitTerminator;
    //     stability:
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //     internal:
    //         SplitInternal yielding (&'a str);
    //     delegate double ended;
    // }

    struct F { c: char }

    type Args = (char,);

    impl FnOnce<Args> for F {
	type Output = bool;

	extern "rust-call" fn call_once(mut self, (c,): Args) -> Self::Output {
	    self.call_mut((c,))
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (c,): Args) -> Self::Output {
	    self.c == c
	}
    }

    #[test]
    fn split_terminator_test1() {
	type P<'a> = &'a str; // P: Pattern<'a>

	let x: &str = "吃葡萄不吐葡萄皮不吃葡萄倒吐葡萄皮";
	let pat: P = "葡萄皮";
	let mut result: SplitTerminator<P> = x.split_terminator::<P>(pat);

	assert_eq!(result.next(), Some::<&str>("吃葡萄不吐"));
	assert_eq!(result.next(), Some::<&str>("不吃葡萄倒吐"));
	assert_eq!(result.next(), None::<&str>);
    }

    #[test]
    fn split_terminator_test2() {
	type P = char; // P: Pattern<'a>

	let x: &str = "吃葡萄不吐葡萄皮不吃葡萄倒吐葡萄皮";
	let pat: P = '皮';
	let mut result: SplitTerminator<P> = x.split_terminator::<P>(pat);

	assert_eq!(result.next(), Some::<&str>("吃葡萄不吐葡萄"));
	assert_eq!(result.next(), Some::<&str>("不吃葡萄倒吐葡萄"));
	assert_eq!(result.next(), None::<&str>);
    }

    #[test]
    fn split_terminator_test3() {
	type P<'a> = &'a [char]; // P: Pattern<'a>

	let x: &str = "吃葡萄不吐葡萄皮不吃葡萄倒吐葡萄皮";
	let pat: P = &['不', '皮'];
	let mut result: SplitTerminator<P> = x.split_terminator::<P>(pat);

	assert_eq!(result.next(), Some::<&str>("吃葡萄"));
	assert_eq!(result.next(), Some::<&str>("吐葡萄"));
	assert_eq!(result.next(), Some::<&str>(""));
	assert_eq!(result.next(), Some::<&str>("吃葡萄倒吐葡萄"));
	assert_eq!(result.next(), None::<&str>);
    }

    #[test]
    fn split_terminator_test4() {
	type P = F; // P: Pattern<'a>

	let x: &str = "吃葡萄不吐葡萄皮不吃葡萄倒吐葡萄皮";
	let pat: P = P { c: '皮' };
	let mut result: SplitTerminator<P> = x.split_terminator::<P>(pat);

	assert_eq!(result.next(), Some::<&str>("吃葡萄不吐葡萄"));
	assert_eq!(result.next(), Some::<&str>("不吃葡萄倒吐葡萄"));
	assert_eq!(result.next(), None::<&str>);
    }

    #[test]
    fn split_terminator_test5() {
	type P<'a, 'b> = &'a &'b str; // P: Pattern<'a>

	let x: &str = "吃葡萄不吐葡萄皮不吃葡萄倒吐葡萄皮";
	let pat: P = &"葡萄皮";
	let mut result: SplitTerminator<P> = x.split_terminator::<P>(pat);

	assert_eq!(result.next(), Some::<&str>("吃葡萄不吐"));
	assert_eq!(result.next(), Some::<&str>("不吃葡萄倒吐"));
	assert_eq!(result.next(), None::<&str>);
    }
}
