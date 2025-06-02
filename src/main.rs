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
    Pair(List, List),
}
/// List is an alias for a reference counted ListE reference
/// We need this as otherwise the borrow check would give us grieve, when we would want to
/// recursively descend into head and tail of lists.
type List = Rc<ListE>;

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
impl fmt::Display for ListE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.internal_fmt(f)?;
        write!(f, ")")
    }
}
/// helper function to construct a literal
pub fn lit(s: &str) -> List {
    Rc::new(ListE::Lit(s.to_owned()))
}
/// helper function to construct an empty list
pub fn empty_list() -> List {
    Rc::new(ListE::Empty())
}
/// cons prepends a new element to a list
pub fn cons(a: List, b: List) -> List {
    Rc::new(ListE::Pair(a, b))
}
/// check if given List is a pair or something else
pub fn is_pair(l: List) -> bool {
    matches!(*l, ListE::Pair(_, _))
}
/// check if given list is empty or something else
pub fn is_empty(l: List) -> bool {
    matches!(*l, ListE::Empty())
}
/// return first (data) element of a list, or the empty list if l is something else than a pair
pub fn car(l: List) -> List {
    // I find this construction, taking a ref of a dereferenced Rc especially ugly.
    // I only found this after googling around. If we do not use this contraption,
    // the compiler complains, that h is moved out of l.
    match &*l {
        ListE::Pair(h, _) => Rc::clone(h),
        _ => empty_list(),
    }
}
/// return tail part of a list or empty if l is not a pair
pub fn cdr(l: List) -> List {
    match &*l {
        ListE::Pair(_, t) => Rc::clone(t),
        _ => Rc::new(ListE::Empty()),
    }
}
/// traverses down a list and computes its size
pub fn length(l: List) -> usize {
    match *l {
        ListE::Empty() => 0,
        _ => 1 + length(cdr(l)),
    }
}
#[macro_export]
macro_rules! list {
    () => { empty_list() };
    ($value:expr) => {
        cons($value, empty_list())
    };
    ($head:expr, $($tail:expr),*) => {
        cons(Rc::new($head), list!($($tail),*))
    };
}
#[macro_export]
macro_rules! lit_list {
    () => { empty_list() };
    ($value:expr) => {
        list!(lit($value))
    };
    ($head:expr, $($tail:expr),*) => {
        cons(lit($head), lit_list!($($tail),*))
    };
}
/// append list b to list a - returns b if a is empty
pub fn append(a: List, b: List) -> List {
    if is_empty(Rc::clone(&a)) {
        b
    } else {
        cons(car(Rc::clone(&a)), append(cdr(a), b))
    }
}
/// reverses the given input list and returns a new list
pub fn reverse(a: List) -> List {
    if is_empty(Rc::clone(&a)) {
        a
    } else {
        append(reverse(cdr(Rc::clone(&a))), list!(car(Rc::clone(&a))))
    }
}

/// dummy main that show cases how to construct and use a list
fn main() {
    let g = lit_list!("Hello", "World", "!");
    println!("{g}")
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{append, car, cdr, length, lit, reverse};

    use super::{cons, empty_list};

    #[test]
    fn empty_list_is_size_0() {
        let l = empty_list();
        assert_eq!(0, length(l));
    }
    #[test]
    fn empty_list_displays_correctly() {
        let l = empty_list();
        assert_eq!("()", format!("{l}"));
    }
    #[test]
    fn cons_1_to_empty_list() {
        let l = cons(lit("1"), empty_list());
        assert_eq!(1, length(l))
    }
    #[test]
    fn cons_1_displays_correctly() {
        let l = cons(lit("1"), empty_list());
        assert_eq!("('1')", format!("{l}"));
    }
    #[test]
    fn cons_2_to_list_of_1_has_size_2() {
        let l = cons(lit("2"), cons(lit("1"), empty_list()));
        assert_eq!(2, length(l))
    }
    #[test]
    fn cons_2_displays_correctly() {
        let l = cons(lit("2"), cons(lit("1"), empty_list()));
        assert_eq!("('2' '1')", format!("{l}"));
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
        let l = empty_list();
        let n = lit_list!("1");
        let a = append(l, n);
        assert_eq!("('1')", format!("{a}"))
    }

    #[test]
    fn append_2_to_1() {
        let l = lit_list!("1");
        let n = lit_list!("2");
        let a = append(Rc::clone(&l), n);
        assert_eq!("('1' '2')", format!("{a}"));
        // the * is necessary as car_rc returns an Rc
        // car_rc is necessary as a itself is an Rc
        assert_eq!(lit("1"), car(a));
    }

    #[test]
    fn real_worldy_list() {
        let l = lit_list!("this", "is", "a", "list");
        assert_eq!("('this' 'is' 'a' 'list')", format!("{l}"))
    }

    #[test]
    fn car_returns_first_element() {
        let l = lit_list!("1");
        let c = car(l);
        assert_eq!("('1')", format!("{c}"))
    }
    #[test]
    fn cdr_return_rest_of_list() {
        let l = lit_list!("1", "2", "3");
        let c = cdr(l);
        assert_eq!("('2' '3')", format!("{c}"))
    }
    #[test]
    fn reverse_reverts_a_lists_order() {
        let l = lit_list!("1", "2", "3");
        let c = reverse(l);
        assert_eq!("('3' '2' '1')", format!("{c}"))
    }
}
