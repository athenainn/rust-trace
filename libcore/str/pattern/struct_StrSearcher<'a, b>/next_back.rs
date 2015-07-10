#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::ReverseSearcher;
    use core::str::pattern::SearchStep::{Match, Reject, Done};
    use core::str::pattern::StrSearcher;
    use core::str::pattern::Pattern;

    // #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    // pub enum SearchStep {
    //     /// Expresses that a match of the pattern has been found at
    //     /// `haystack[a..b]`.
    //     Match(usize, usize),
    //     /// Expresses that `haystack[a..b]` has been rejected as a possible match
    //     /// of the pattern.
    //     ///
    //     /// Note that there might be more than one `Reject` between two `Match`es,
    //     /// there is no requirement for them to be combined into one.
    //     Reject(usize, usize),
    //     /// Expresses that every byte of the haystack has been visted, ending
    //     /// the iteration.
    //     Done
    // }

    // #[derive(Clone)]
    // pub struct StrSearcher<'a, 'b> {
    //     haystack: &'a str,
    //     needle: &'b str,
    //     start: usize,
    //     end: usize,
    //     state: State,
    // }

    // #[derive(Clone, PartialEq)]
    // enum State { Done, NotDone, Reject(usize, usize) }
    // impl State {
    //     #[inline] fn done(&self) -> bool { *self == State::Done }
    //     #[inline] fn take(&mut self) -> State { ::mem::replace(self, State::NotDone) }
    // }

    // pub trait Pattern<'a>: Sized {
    //     /// Associated searcher for this pattern
    //     type Searcher: Searcher<'a>;
    //
    //     /// Constructs the associated searcher from
    //     /// `self` and the `haystack` to search in.
    //     fn into_searcher(self, haystack: &'a str) -> Self::Searcher;
    //
    //     /// Checks whether the pattern matches anywhere in the haystack
    //     #[inline]
    //     fn is_contained_in(self, haystack: &'a str) -> bool {
    //         self.into_searcher(haystack).next_match().is_some()
    //     }
    //
    //     /// Checks whether the pattern matches at the front of the haystack
    //     #[inline]
    //     fn is_prefix_of(self, haystack: &'a str) -> bool {
    //         match self.into_searcher(haystack).next() {
    //             SearchStep::Match(0, _) => true,
    //             _ => false,
    //         }
    //     }
    //
    //     /// Checks whether the pattern matches at the back of the haystack
    //     //     #[inline]
    //     fn is_suffix_of(self, haystack: &'a str) -> bool
    //         where Self::Searcher: ReverseSearcher<'a>
    //     {
    //         match self.into_searcher(haystack).next_back() {
    //             SearchStep::Match(_, j) if haystack.len() == j => true,
    //             _ => false,
    //         }
    //     }
    // }

    // impl<'a, 'b> Pattern<'a> for &'b str {
    //     type Searcher = StrSearcher<'a, 'b>;
    //
    //     #[inline]
    //     fn into_searcher(self, haystack: &'a str) -> StrSearcher<'a, 'b> {
    //         StrSearcher {
    //             haystack: haystack,
    //             needle: self,
    //             start: 0,
    //             end: haystack.len(),
    //             state: State::NotDone,
    //         }
    //     }
    // }

    // pub unsafe trait Searcher<'a> {
    //     /// Getter for the underlaying string to be searched in
    //     ///
    //     /// Will always return the same `&str`
    //     fn haystack(&self) -> &'a str;
    //
    //     /// Performs the next search step starting from the front.
    //     ///
    //     /// - Returns `Match(a, b)` if `haystack[a..b]` matches the pattern.
    //     /// - Returns `Reject(a, b)` if `haystack[a..b]` can not match the
    //     ///   pattern, even partially.
    //     /// - Returns `Done` if every byte of the haystack has been visited
    //     ///
    //     /// The stream of `Match` and `Reject` values up to a `Done`
    //     /// will contain index ranges that are adjacent, non-overlapping,
    //     /// covering the whole haystack, and laying on utf8 boundaries.
    //     ///
    //     /// A `Match` result needs to contain the whole matched pattern,
    //     /// however `Reject` results may be split up into arbitrary
    //     /// many adjacent fragments. Both ranges may have zero length.
    //     ///
    //     /// As an example, the pattern `"aaa"` and the haystack `"cbaaaaab"`
    //     /// might produce the stream
    //     /// `[Reject(0, 1), Reject(1, 2), Match(2, 5), Reject(5, 8)]`
    //     fn next(&mut self) -> SearchStep;
    //
    //     /// Find the next `Match` result. See `next()`
    //     #[inline]
    //     fn next_match(&mut self) -> Option<(usize, usize)> {
    //         loop {
    //             match self.next() {
    //                 SearchStep::Match(a, b) => return Some((a, b)),
    //                 SearchStep::Done => return None,
    //                 _ => continue,
    //             }
    //         }
    //     }
    //
    //     /// Find the next `Reject` result. See `next()`
    //     #[inline]
    //     fn next_reject(&mut self) -> Option<(usize, usize)> {
    //         loop {
    //             match self.next() {
    //                 SearchStep::Reject(a, b) => return Some((a, b)),
    //                 SearchStep::Done => return None,
    //                 _ => continue,
    //             }
    //         }
    //     }
    // }

    // pub unsafe trait ReverseSearcher<'a>: Searcher<'a> {
    //     /// Performs the next search step starting from the back.
    //     ///
    //     /// - Returns `Match(a, b)` if `haystack[a..b]` matches the pattern.
    //     /// - Returns `Reject(a, b)` if `haystack[a..b]` can not match the
    //     ///   pattern, even partially.
    //     /// - Returns `Done` if every byte of the haystack has been visited
    //     ///
    //     /// The stream of `Match` and `Reject` values up to a `Done`
    //     /// will contain index ranges that are adjacent, non-overlapping,
    //     /// covering the whole haystack, and laying on utf8 boundaries.
    //     ///
    //     /// A `Match` result needs to contain the whole matched pattern,
    //     /// however `Reject` results may be split up into arbitrary
    //     /// many adjacent fragments. Both ranges may have zero length.
    //     ///
    //     /// As an example, the pattern `"aaa"` and the haystack `"cbaaaaab"`
    //     /// might produce the stream
    //     /// `[Reject(7, 8), Match(4, 7), Reject(1, 4), Reject(0, 1)]`
    //     fn next_back(&mut self) -> SearchStep;
    //
    //     /// Find the next `Match` result. See `next_back()`
    //     #[inline]
    //     fn next_match_back(&mut self) -> Option<(usize, usize)>{
    //         loop {
    //             match self.next_back() {
    //                 SearchStep::Match(a, b) => return Some((a, b)),
    //                 SearchStep::Done => return None,
    //                 _ => continue,
    //             }
    //         }
    //     }
    //
    //     /// Find the next `Reject` result. See `next_back()`
    //     #[inline]
    //     fn next_reject_back(&mut self) -> Option<(usize, usize)>{
    //         loop {
    //             match self.next_back() {
    //                 SearchStep::Reject(a, b) => return Some((a, b)),
    //                 SearchStep::Done => return None,
    //                 _ => continue,
    //             }
    //         }
    //     }
    // }

    // unsafe impl<'a, 'b> ReverseSearcher<'a> for StrSearcher<'a, 'b>  {
    //     #[inline]
    //     fn next_back(&mut self) -> SearchStep {
    //         str_search_step(self,
    //         |m: &mut StrSearcher| {
    //             // Backward step for empty needle
    //             let current_end = m.end;
    //             if !m.state.done() {
    //                 m.end = m.haystack.char_range_at_reverse(current_end).next;
    //                 m.state = State::Reject(m.end, current_end);
    //             }
    //             SearchStep::Match(current_end, current_end)
    //         },
    //         |m: &mut StrSearcher| {
    //             // Backward step for nonempty needle
    //             let current_end = m.end;
    //             // Compare byte window because this might break utf8 boundaries
    //             let possible_match = &m.haystack.as_bytes()[m.end - m.needle.len() .. m.end];
    //             if possible_match == m.needle.as_bytes() {
    //                 m.end -= m.needle.len();
    //                 SearchStep::Match(m.end, current_end)
    //             } else {
    //                 // Skip a char
    //                 let haystack_prefix = &m.haystack[..m.end];
    //                 m.end -= haystack_prefix.chars().rev().next().unwrap().len_utf8();
    //                 SearchStep::Reject(m.end, current_end)
    //             }
    //         })
    //     }
    // }

    #[test]
    fn next_back_test1() {
	let string: &str = "玻璃";
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let mut searcher: StrSearcher = string.into_searcher(haystack);

	assert_eq!(searcher.next_back(), Reject(33, 36));
	assert_eq!(searcher.next_back(), Reject(30, 33));
	assert_eq!(searcher.next_back(), Reject(27, 30));
	assert_eq!(searcher.next_back(), Reject(24, 27));
	assert_eq!(searcher.next_back(), Reject(21, 24));
	assert_eq!(searcher.next_back(), Reject(18, 21));
	assert_eq!(searcher.next_back(), Match(12, 18));
	assert_eq!(searcher.next_back(), Reject(9, 12));
	assert_eq!(searcher.next_back(), Reject(6, 9));
	assert_eq!(searcher.next_back(), Reject(3, 6));
	assert_eq!(searcher.next_back(), Reject(0, 3));
	assert_eq!(searcher.next_back(), Done);
    }
}
