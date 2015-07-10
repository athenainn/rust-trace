#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::Searcher;
    use core::str::pattern::StrSearcher;
    use core::str::pattern::Pattern;

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

    // unsafe impl<'a, 'b> Searcher<'a> for StrSearcher<'a, 'b>  {
    //     #[inline]
    //     fn haystack(&self) -> &'a str {
    //         self.haystack
    //     }
    //
    //     #[inline]
    //     fn next(&mut self) -> SearchStep {
    //         str_search_step(self,
    //         |m: &mut StrSearcher| {
    //             // Forward step for empty needle
    //             let current_start = m.start;
    //             if !m.state.done() {
    //                 m.start = m.haystack.char_range_at(current_start).next;
    //                 m.state = State::Reject(current_start, m.start);
    //             }
    //             SearchStep::Match(current_start, current_start)
    //         },
    //         |m: &mut StrSearcher| {
    //             // Forward step for nonempty needle
    //             let current_start = m.start;
    //             // Compare byte window because this might break utf8 boundaries
    //             let possible_match = &m.haystack.as_bytes()[m.start .. m.start + m.needle.len()];
    //             if possible_match == m.needle.as_bytes() {
    //                 m.start += m.needle.len();
    //                 SearchStep::Match(current_start, m.start)
    //             } else {
    //                 // Skip a char
    //                 let haystack_suffix = &m.haystack[m.start..];
    //                 m.start += haystack_suffix.chars().next().unwrap().len_utf8();
    //                 SearchStep::Reject(current_start, m.start)
    //             }
    //         })
    //     }
    // }

    #[test]
    fn next_reject_test1() {
	let string: &str = "玻璃";
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let mut searcher: StrSearcher = string.into_searcher(haystack);

	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((0, 3)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((3, 6)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((6, 9)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((9, 12)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((18, 21)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((21, 24)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((24, 27)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((27, 30)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((30, 33)));
	assert_eq!(searcher.next_reject(), Some::<(usize, usize)>((33, 36)));
	assert_eq!(searcher.next_reject(), None::<(usize, usize)>);
    }
}
