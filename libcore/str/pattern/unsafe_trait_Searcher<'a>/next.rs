#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::Searcher;
    use core::str::pattern::SearchStep::{self, Match, Reject, Done};

    use core::str::CharIndices;

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

    trait CharEq {
	fn matches(&mut self, char) -> bool;
	fn only_ascii(&self) -> bool;
    }

    impl CharEq for char {
	#[inline]
	fn matches(&mut self, c: char) -> bool { *self == c }

	#[inline]
	fn only_ascii(&self) -> bool { (*self as u32) < 0xf0 }
    }

    #[derive(Clone)]
    struct CharEqSearcher<'a, C: CharEq> {
	char_eq: C,
	haystack: &'a str,
	char_indices: CharIndices<'a>,
	ascii_only: bool
    }

    unsafe impl<'a, C: CharEq> Searcher<'a> for CharEqSearcher<'a, C> {
	#[inline]
	fn haystack(&self) -> &'a str {
	    self.haystack
	}

	#[inline]
	fn next(&mut self) -> SearchStep {
	    let s: &mut CharIndices<'a> = &mut self.char_indices;
	    let pre_len: usize = s.size_hint().1.unwrap();

	    if let Some::<(usize, char)>((i, c)) = s.next() {
		let len: usize = s.size_hint().1.unwrap();
		let char_len = pre_len - len;

		if self.char_eq.matches(c) {
		    return SearchStep::Match(i, i + char_len);
		} else {
		    return SearchStep::Reject(i, i + char_len);
		}
	    }

	    SearchStep::Done
	}

	// #[inline]
	// fn next_match(&mut self) -> Option<(usize, usize)> {
	//     loop {
	//         match self.next() {
	//             SearchStep::Match(a, b) => return Some((a, b)),
	//             SearchStep::Done => return None,
	//             _ => continue,
	//         }
	//     }
	// }

	// #[inline]
	// fn next_reject(&mut self) -> Option<(usize, usize)> {
	//     loop {
	//         match self.next() {
	//             SearchStep::Reject(a, b) => return Some((a, b)),
	//             SearchStep::Done => return None,
	//             _ => continue,
	//         }
	//     }
	// }
    }

    #[test]
    fn next_test1() {
	let c: char = '能';
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let char_indices: CharIndices = haystack.char_indices();
	let mut searcher: CharEqSearcher<char> = CharEqSearcher {
	    char_eq: c,
	    haystack: haystack,
	    char_indices: char_indices,
	    ascii_only: c.only_ascii()
	};

	assert_eq!(searcher.next(), Reject(0, 3));
	assert_eq!(searcher.next(), Match(3, 6));
	assert_eq!(searcher.next(), Reject(6, 9));
	assert_eq!(searcher.next(), Reject(9, 12));
	assert_eq!(searcher.next(), Reject(12, 15));
	assert_eq!(searcher.next(), Reject(15, 18));
	assert_eq!(searcher.next(), Reject(18, 21));
	assert_eq!(searcher.next(), Reject(21, 24));
	assert_eq!(searcher.next(), Reject(24, 27));
	assert_eq!(searcher.next(), Reject(27, 30));
	assert_eq!(searcher.next(), Reject(30, 33));
	assert_eq!(searcher.next(), Reject(33, 36));
	assert_eq!(searcher.next(), Done);
    }
}
