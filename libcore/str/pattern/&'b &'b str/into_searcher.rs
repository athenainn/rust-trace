#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::Searcher;
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

    // macro_rules! pattern_methods {
    //     ($t:ty, $pmap:expr, $smap:expr) => {
    //         type Searcher = $t;
    //
    //         #[inline]
    //         fn into_searcher(self, haystack: &'a str) -> $t {
    //             ($smap)(($pmap)(self).into_searcher(haystack))
    //         }
    //
    //         #[inline]
    //         fn is_contained_in(self, haystack: &'a str) -> bool {
    //             ($pmap)(self).is_contained_in(haystack)
    //         }
    //
    //         #[inline]
    //         fn is_prefix_of(self, haystack: &'a str) -> bool {
    //             ($pmap)(self).is_prefix_of(haystack)
    //         }
    //
    //         #[inline]
    //         fn is_suffix_of(self, haystack: &'a str) -> bool
    //             where $t: ReverseSearcher<'a>
    //         {
    //             ($pmap)(self).is_suffix_of(haystack)
    //         }
    //     }
    // }

    // impl<'a, 'b> Pattern<'a> for &'b &'b str {
    //     pattern_methods!(StrSearcher<'a, 'b>, |&s| s, |s| s);
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
    fn into_searcher_test1() {
	let string: &str = "玻璃";
	let string_ptr: &&str = &string;
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let mut searcher: StrSearcher = string_ptr.into_searcher(haystack);

	assert_eq!(searcher.next(), Reject(0, 3));
	assert_eq!(searcher.next(), Reject(3, 6));
	assert_eq!(searcher.next(), Reject(6, 9));
	assert_eq!(searcher.next(), Reject(9, 12));
	assert_eq!(searcher.next(), Match(12, 18));
	assert_eq!(searcher.next(), Reject(18, 21));
	assert_eq!(searcher.next(), Reject(21, 24));
	assert_eq!(searcher.next(), Reject(24, 27));
	assert_eq!(searcher.next(), Reject(27, 30));
	assert_eq!(searcher.next(), Reject(30, 33));
	assert_eq!(searcher.next(), Reject(33, 36));
	assert_eq!(searcher.next(), Done);
    }
}
