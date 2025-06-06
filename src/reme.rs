//! reme is my interpretation of a `rust scheme`
//!
//! this implementation is an exploration of how you would implement a scheme like language in rust
//! using only an enum and helper functions and macros.
//!
//! this little project was inspired and motivated by [mato](github.com/sschober/mato) - my
//! markdown transformer framework. the syntax trees the parser produces are very strangely formed
//! and hard to read. that's why i wanted to explorer a more academical approach.
//!
use std::{fmt, rc::Rc};

/// the List enum is the base type of our little language.
/// its `Pair` variant serves to form trees (and in consequence and by convention) list.
/// the `Empty` variant terminates a list and `Lit`s capture values. here, we could have a bit more
/// power (Numbers)
#[derive(Debug, Clone, PartialEq)]
pub enum ListE {
    Empty(),
    Lit(String),
    Pair(ListR, ListR),
}
/// List is an alias for a reference counted ListE reference
/// We need this as otherwise the borrow checker would give us grieve, when we would want to
/// recursively descend into head and tail of lists.
type ListR = Rc<ListE>;

/// List is a newtype wrapper around our ListR type alias
/// This enables nice export/import properties
#[derive(Debug, PartialEq)]
pub struct List(ListR);

#[macro_export]
macro_rules! list {
    () => { List::empty() };
    ($value:expr) => {
        $value.cons(List::empty())
    };
    ($head:expr, $($tail:expr),*) => {
        cons(Rc::new($head), list!($($tail),*))
    };
}
#[macro_export]
macro_rules! lit_list {
    () => { List::empty()};
    ($value:expr) => {
        list!(List::lit($value))
    };
    ($head:expr, $($tail:expr),*) => {
        List::lit($head).cons(lit_list!($($tail),*))
    };
}
impl ListE {
    fn internal_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListE::Empty() => write!(f, ""),
            ListE::Lit(s) => write!(f, "'{}'", s),
            ListE::Pair(h, t) => {
                (*h).internal_fmt(f)?;
                match **t {
                    ListE::Empty() => write!(f, "")?,
                    _ => write!(f, " ")?,
                };
                (*t).internal_fmt(f)
            }
        }
    }
}
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.0.internal_fmt(f)?;
        write!(f, ")")
    }
}
impl fmt::Display for ListE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.internal_fmt(f)?;
        write!(f, ")")
    }
}
impl List {
    /// helper function to construct a literal
    pub fn lit(s: &str) -> Self {
        List(Rc::new(ListE::Lit(s.to_owned())))
    }
    /// helper function to construct an empty list
    pub fn empty() -> List {
        List(Rc::new(ListE::Empty()))
    }
    /// cons prepends a new element to a list
    pub fn cons(self, b: List) -> List {
        List(Rc::new(ListE::Pair(self.0, b.0)))
    }
    /// check if given list is empty or something else
    pub fn is_empty(&self) -> bool {
        matches!(*self.0, ListE::Empty())
    }
    /// return first (data) element of a list, or the empty list if l is something else than a pair
    pub fn car(&self) -> List {
        // I find this construction, taking a ref of a dereferenced Rc especially ugly.
        // I only found this after googling around. If we do not use this contraption,
        // the compiler complains, that h is moved out of l.
        match &*self.0 {
            ListE::Pair(h, _) => List(Rc::clone(h)),
            _ => List::empty(),
        }
    }
    /// return tail part of a list or empty if l is not a pair
    pub fn cdr(&self) -> List {
        match &*self.0 {
            ListE::Pair(_, t) => List(Rc::clone(t)),
            _ => List::empty(),
        }
    }
    /// append list b to list a - returns b if a is empty
    pub fn append(self, b: List) -> List {
        if self.is_empty() {
            b
        } else {
            self.car().cons(self.cdr().append(b))
        }
    }
    pub fn reverse(self) -> List {
        if self.is_empty() {
            self
        } else {
            self.cdr().reverse().append(list!(self.car()))
        }
    }
    pub fn length(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1 + self.cdr().length()
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::List;

    #[test]
    fn empty_list_is_size_0() {
        let l = List::empty();
        assert_eq!(0, l.length());
    }
    #[test]
    fn empty_list_displays_correctly() {
        let l = List::empty();
        assert_eq!("()", format!("{l}"));
    }

    #[test]
    fn list_marcro_works_for_empty() {
        let l = lit_list!();
        assert_eq!("()", format!("{l}"));
    }

    #[test]
    fn list_marcro_works_for_1() {
        let l = lit_list!("1");
        assert_eq!("('1')", format!("{l}"));
    }

    #[test]
    fn list_marcro_works_for_2() {
        let l = lit_list!("1", "2");
        assert_eq!("('1' '2')", format!("{l}"));
    }

    #[test]
    fn append_1_to_empty() {
        let l = List::empty();
        let n = lit_list!("1");
        let a = l.append(n);
        assert_eq!("('1')", format!("{a}"))
    }

    #[test]
    fn append_2_to_1() {
        let l = lit_list!("1");
        let n = lit_list!("2");
        let a = l.append(n);
        assert_eq!("('1' '2')", format!("{a}"));
        assert_eq!(List::lit("1"), a.car());
    }

    #[test]
    fn real_worldy_list() {
        let l = lit_list!("this", "is", "a", "list");
        assert_eq!("('this' 'is' 'a' 'list')", format!("{l}"))
    }

    #[test]
    fn car_returns_first_element() {
        let l = lit_list!("1");
        let c = l.car();
        assert_eq!("('1')", format!("{c}"))
    }
    #[test]
    fn cdr_return_rest_of_list() {
        let l = lit_list!("1", "2", "3");
        let c = l.cdr();
        assert_eq!("('2' '3')", format!("{c}"))
    }
    #[test]
    fn reverse_reverts_a_lists_order() {
        let l = lit_list!("1", "2", "3");
        let c = l.reverse();
        assert_eq!("('3' '2' '1')", format!("{c}"))
    }
}
