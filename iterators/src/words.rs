/*
Rust standard collections
all Rust standard collections implement into_iter(), but return different
types depending on whether it's applied on a reference or a value:

Variable type   -   Type returned by into_iter()    -   Type returned by next()
--------------------------------------------------------------------------------
Vec<T>	        -   std::iter::IntoIterator<T>	    -   T
&Vec<T>	        -   std::slice::Iter<T>	            -   &T
&mut Vec<T>	    -   std::slice::IterMut<T>	        -   &mut T
--------------------------------------------------------------------------------
*/

#[derive(Debug)]
pub struct Words<'a> {
    words: Vec<&'a str>
}

impl<'a> Words<'a> {
    pub fn new(init: &str) -> Words {
        Words {
            words: init.split(" ").collect(),
        }
    }

    pub fn iter(&'a self) -> WordsHelper<'a> {
        self.into_iter()
    }

    pub fn iter_mut(&'a mut self) -> MutWordsHelper<'a> {
        self.into_iter()
    }
}

// =========================================================
// ================= Consuming iterator ====================
// =========================================================

// helper struct 
pub struct WordsIterator<'a> {
    iter: ::std::vec::IntoIter<&'a str>,
}

impl<'a> Iterator for WordsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


impl<'a> IntoIterator for Words<'a> {
    type Item = &'a str;
    type IntoIter = WordsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        WordsIterator {
            iter: self.words.into_iter(),
        }
    }
}

// =========================================================
// =============== Non consuming iterator ==================
// =========================================================

// helper struct 
pub struct WordsHelper<'a> {
    iter: ::std::slice::Iter<'a, &'a str>,
}

impl<'a> Iterator for WordsHelper<'a> {
    type Item = &'a &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> IntoIterator for &'a Words<'a> {
    type Item = &'a &'a str;
    type IntoIter = WordsHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        WordsHelper {
            iter: self.words.iter(),
        }
    }
}

// =========================================================
// =========== Mutable non consuming iterator ==============
// =========================================================

// helper struct 
pub struct MutWordsHelper<'a> {
    iter: ::std::slice::IterMut<'a, &'a str>,
}

impl<'a> Iterator for MutWordsHelper<'a> {
    type Item = &'a mut &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> IntoIterator for &'a mut Words<'a> {
    type Item = &'a mut &'a str;
    type IntoIter = MutWordsHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MutWordsHelper {
            iter: self.words.iter_mut(),
        }
    }
}

use std::iter::FromIterator;

impl<'a> FromIterator<&'a str> for Words<'a> {

    fn from_iter<T>(iter: T) -> Self 
    where T: IntoIterator<Item = &'a str> {
        Words {
            words: iter.into_iter().collect(),
        }
    }
}