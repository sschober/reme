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
pub enum List {
    Empty(),
    Lit(String),
    Pair(Rc<List>, Rc<List>),
}
impl List {
    fn internal_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Empty() => write!(f, ""),
            List::Lit(s) => write!(f, "'{}'", s),
            List::Pair(h, t) => {
                (*h).internal_fmt(f)?;
                match **t {
                    List::Empty() => write!(f, "")?,
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
        self.internal_fmt(f)?;
        write!(f, ")")
    }
}

pub fn lit(s: &str) -> List {
    List::Lit(s.to_owned())
}

#[macro_export]
macro_rules! list_rc {
    () => { Rc::new(list()) };
    ($value:expr) => {
        cons_rc($value, Rc::new(list()))
    };
    ($head:expr, $($tail:expr),*) => {
        cons_rc(Rc::new($head), list_rc!($($tail),*))
    };
}
#[macro_export]
macro_rules! lit_list_rc {
    () => { Rc::new(list()) };
    ($value:expr) => {
        cons_rc(Rc::new(lit($value)), Rc::new(list()))
    };
    ($head:expr, $($tail:expr),*) => {
        cons_rc(Rc::new(lit($head)), lit_list_rc!($($tail),*))
    };
}
/// cons prepends a new element to a list
pub fn cons(a: List, b: List) -> List {
    List::Pair(Rc::new(a), Rc::new(b))
}
pub fn cons_rc(a: Rc<List>, b: Rc<List>) -> Rc<List> {
    Rc::new(List::Pair(a, b))
}
pub fn is_pair(l: List) -> bool {
    matches!(l, List::Pair(_, _))
}
pub fn is_empty_rc(l: Rc<List>) -> bool {
    matches!(*l, List::Empty())
}
pub fn car_rc(l: Rc<List>) -> Rc<List> {
    // I find this construction, taking a ref of a dereferenced Rc especially ugly.
    // I only found this after googling around. If we do not use this contraption,
    // the compiler complains, that h is moved out of l.
    match &*l {
        List::Pair(h, _) => Rc::clone(h),
        _ => Rc::new(List::Empty()),
    }
}
pub fn cdr_rc(l: Rc<List>) -> Rc<List> {
    match &*l {
        List::Pair(_, t) => Rc::clone(t),
        _ => Rc::new(List::Empty()),
    }
}
pub fn car(l: &List) -> &List {
    match l {
        List::Pair(h, _) => h,
        _ => &List::Empty(),
    }
}
pub fn cdr(l: &List) -> &List {
    match l {
        List::Pair(_, t) => t,
        _ => &List::Empty(),
    }
}
pub fn list() -> List {
    List::Empty()
}
pub fn list_rc() -> Rc<List> {
    Rc::new(list())
}
pub fn length(l: &List) -> usize {
    match l {
        List::Empty() => 0,
        _ => 1 + length(cdr(l)),
    }
}
/// append appends list b to list a
pub fn append(a: Rc<List>, b: Rc<List>) -> Rc<List> {
    match *a {
        List::Empty() => b,
        _ => cons_rc(car_rc(Rc::clone(&a)), append(cdr_rc(a), b)),
    }
}

pub fn reverse(a: Rc<List>) -> Rc<List> {
    if is_empty_rc(Rc::clone(&a)) {
        a
    } else {
        append(
            reverse(cdr_rc(Rc::clone(&a))),
            list_rc!(car_rc(Rc::clone(&a))),
        )
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{append, car_rc, cdr_rc, length, lit, reverse};

    use super::{cons, cons_rc, list};

    #[test]
    fn empty_list_is_size_0() {
        let l = list();
        assert_eq!(0, length(&l));
    }
    #[test]
    fn empty_list_displays_correctly() {
        let l = list();
        assert_eq!("()", format!("{l}"));
    }
    #[test]
    fn cons_1_to_empty_list() {
        let l = cons(lit("1"), list());
        assert_eq!(1, length(&l))
    }
    #[test]
    fn cons_1_displays_correctly() {
        let l = cons(lit("1"), list());
        assert_eq!("('1')", format!("{l}"));
    }
    #[test]
    fn cons_2_to_list_of_1_has_size_2() {
        let l = cons(lit("2"), cons(lit("1"), list()));
        assert_eq!(2, length(&l))
    }
    #[test]
    fn cons_2_displays_correctly() {
        let l = cons(lit("2"), cons(lit("1"), list()));
        assert_eq!("('2' '1')", format!("{l}"));
    }

    #[test]
    fn list_marcro_works_for_empty() {
        let l = lit_list_rc!();
        assert_eq!("()", format!("{l}"));
    }

    #[test]
    fn list_marcro_works_for_1() {
        let l = lit_list_rc!("1");
        assert_eq!("('1')", format!("{l}"));
    }

    #[test]
    fn list_marcro_works_for_2() {
        let l = lit_list_rc!("1", "2");
        eprintln!("{l}");
        eprintln!("{l:?}");
        assert_eq!("('1' '2')", format!("{l}"));
    }

    #[test]
    fn append_1_to_empty() {
        let l = list();
        let n = lit_list_rc!("1");
        let a = append(Rc::new(l), n);
        eprintln!("a: {a}");
        eprintln!("a: {a:?}");
        assert_eq!("('1')", format!("{a}"))
    }

    #[test]
    fn append_2_to_1() {
        let l = lit_list_rc!("1");
        let n = lit_list_rc!("2");
        let a = append(Rc::clone(&l), n);

        eprintln!("a: {a}");
        eprintln!("a: {a:?}");
        assert_eq!("('1' '2')", format!("{a}"));
        // the * is necessary as car_rc returns an Rc
        // car_rc is necessary as a itself is an Rc
        assert_eq!(lit("1"), *car_rc(a));
    }

    #[test]
    fn real_worldy_list() {
        let l = lit_list_rc!("this", "is", "a", "list");

        eprintln!("l: {l}");
        eprintln!("l: {l:?}");
        assert_eq!("('this' 'is' 'a' 'list')", format!("{l}"))
    }

    #[test]
    fn car_returns_first_element() {
        let l = lit_list_rc!("1");
        let c = car_rc(l);
        eprintln!("c: {c}");
        eprintln!("c: {c:?}");
        assert_eq!("('1')", format!("{c}"))
    }
    #[test]
    fn cdr_return_rest_of_list() {
        let l = lit_list_rc!("1", "2", "3");
        eprintln!("l: {l}");
        eprintln!("l: {l:?}");
        let c = cdr_rc(l);
        eprintln!("c: {c}");
        eprintln!("c: {c:?}");
        assert_eq!("('2' '3')", format!("{c}"))
    }
    #[test]
    fn reverse_reverts_a_lists_order() {
        let l = lit_list_rc!("1", "2", "3");
        eprintln!("l: {l}");
        eprintln!("l: {l:?}");
        let c = reverse(l);
        eprintln!("c: {c}");
        eprintln!("c: {c:?}");
        assert_eq!("('3' '2' '1')", format!("{c}"))
    }
}
