use std::fmt::{self};

pub enum List {
    Empty(),
    Lit(String),
    Pair(Box<List>, Box<List>),
}
impl List {
    fn internal_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Empty() => {
                write!(f, "")
            }
            List::Lit(s) => write!(f, "'{}'", s),
            List::Pair(h, t) => {
                _ = (*h).internal_fmt(f);
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
        _ = write!(f, "(");
        _ = self.internal_fmt(f);
        write!(f, ")")
    }
}

pub fn lit(s: &str) -> List {
    List::Lit(s.to_owned())
}

#[macro_export]
macro_rules! list {
    () => { list() };
    ($value:expr) => {
        // TODO we might drop the lit as the caller should decide, what we want to add
        cons(lit($value), list())
    };
    ($head:expr, $($tail:expr),*) => {
        // TODO we might drop the lit as the caller should decide, what we want to add
        cons(lit($head), list!($($tail),*))
    };
}
pub fn cons(a: List, b: List) -> List {
    List::Pair(Box::new(a), Box::new(b))
}
pub fn is_pair(l: List) -> bool {
    matches!(l, List::Pair(_, _))
}
pub fn car(l: List) -> List {
    match l {
        List::Pair(h, _) => *h,
        _ => List::Empty(),
    }
}
pub fn cdr(l: List) -> List {
    match l {
        List::Pair(_, t) => *t,
        _ => List::Empty(),
    }
}
pub fn list() -> List {
    List::Empty()
}
pub fn length(l: List) -> usize {
    match l {
        List::Empty() => 0,
        _ => 1 + length(cdr(l)),
    }
}
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use crate::{length, lit};

    use super::{cons, list};

    #[test]
    fn empty_list_is_size_0() {
        let l = list();
        assert_eq!(0, length(l));
    }
    #[test]
    fn empty_list_displays_correctly() {
        let l = list();
        assert_eq!("()", format!("{}", l));
    }
    #[test]
    fn cons_1_to_empty_list() {
        let l = cons(lit("1"), list());
        assert_eq!(1, length(l))
    }
    #[test]
    fn cons_1_displays_correctly() {
        let l = cons(lit("1"), list());
        assert_eq!("('1')", format!("{}", l));
    }
    #[test]
    fn cons_2_to_list_of_1_has_size_2() {
        let l = cons(lit("2"), cons(lit("1"), list()));
        assert_eq!(2, length(l))
    }
    #[test]
    fn cons_2_displays_correctly() {
        let l = cons(lit("2"), cons(lit("1"), list()));
        assert_eq!("('2' '1')", format!("{}", l));
    }

    #[test]
    fn list_marcro_works_for_empty() {
        let l = list!();
        assert_eq!("()", format!("{}", l));
    }

    #[test]
    fn list_marcro_works_for_1() {
        let l = list!("1");
        assert_eq!("('1')", format!("{}", l));
    }

    #[test]
    fn list_marcro_works_for_2() {
        let l = list!("1", "2");
        assert_eq!("('1' '2')", format!("{}", l));
    }
}
