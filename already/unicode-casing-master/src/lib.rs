#![no_std]
mod bool_trie;
mod tables;

use tables::{conversions, general_category};

pub trait CharExt {
    /// Indicates whether a character is titlecased.
    ///
    /// 'Titlecase' is defined in terms of the Unicode General Category
    /// 'Lt'.
    fn is_titlecase(self) -> bool;

    /// Converts a character to its titlecase equivalent.
    ///
    /// This performs complex unconditional mappings with no tailoring.
    /// See `to_uppercase()` for references and more information.
    ///
    /// This differs from `to_uppercase()` since Unicode contains
    /// digraphs and ligature characters.
    /// For example, U+01F3 “ǳ” and U+FB01 “ﬁ”
    /// map to U+01F1 “Ǳ” and U+0046 U+0069 “Fi”, respectively.
    ///
    /// # Return value
    ///
    /// Returns an iterator which yields the characters corresponding to the
    /// titlecase equivalent of the character. If no conversion is possible then
    /// an iterator with just the input character is returned.
    ///
    /// Note that `is_titlecase` will not necessarily return `true` for the
    /// yielded characters.
    fn to_titlecase(self) -> ToTitlecase;
}

impl CharExt for char {
    #[inline]
    fn is_titlecase(self) -> bool {
        general_category::Lt(self)
    }

    #[inline]
    fn to_titlecase(self) -> ToTitlecase {
        ToTitlecase(CaseMappingIter::new(conversions::to_title(self)))
    }
}

enum CaseMappingIter {
    Three(char, char, char),
    Two(char, char),
    One(char),
    Zero,
}

impl Iterator for CaseMappingIter {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        match *self {
            CaseMappingIter::Three(a, b, c) => {
                *self = CaseMappingIter::Two(b, c);
                Some(a)
            }
            CaseMappingIter::Two(b, c) => {
                *self = CaseMappingIter::One(c);
                Some(b)
            }
            CaseMappingIter::One(c) => {
                *self = CaseMappingIter::Zero;
                Some(c)
            }
            CaseMappingIter::Zero => None,
        }
    }
}

impl CaseMappingIter {
    fn new(chars: [char; 3]) -> CaseMappingIter {
        if chars[2] == '\0' {
            if chars[1] == '\0' {
                CaseMappingIter::One(chars[0]) // Including if chars[0] == '\0'
            } else {
                CaseMappingIter::Two(chars[0], chars[1])
            }
        } else {
            CaseMappingIter::Three(chars[0], chars[1], chars[2])
        }
    }
}

pub struct ToTitlecase(CaseMappingIter);

impl Iterator for ToTitlecase {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_titlecase() {
        assert!(!'A'.is_titlecase());
        assert!('\u{1FFC}'.is_titlecase());
        assert!('ǅ'.is_titlecase());
    }

    #[test]
    fn test_to_titlecase() {
        fn title(c: char) -> Vec<char> {
            c.to_titlecase().collect()
        }
        assert_eq!(title('a'), ['A']);
        assert_eq!(title('ö'), ['Ö']);
        assert_eq!(title('ß'), ['S', 's']); // not ẞ: Latin capital letter sharp s
        assert_eq!(title('ü'), ['Ü']);
        assert_eq!(title('💩'), ['💩']);

        assert_eq!(title('σ'), ['Σ']);
        assert_eq!(title('τ'), ['Τ']);
        assert_eq!(title('ι'), ['Ι']);
        assert_eq!(title('γ'), ['Γ']);
        assert_eq!(title('μ'), ['Μ']);
        assert_eq!(title('α'), ['Α']);
        assert_eq!(title('ς'), ['Σ']);
        assert_eq!(title('Ǆ'), ['ǅ']);
        assert_eq!(title('ﬁ'), ['F', 'i']);
        assert_eq!(title('ᾀ'), ['ᾈ']);
    }
}
