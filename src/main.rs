mod reme;
use crate::reme::List;
use crate::reme::{cons, empty_list, lit};
/// dummy main that show cases how to construct and use a list
fn main() {
    let g = lit_list!("Hello", "World", "!");
    println!("{g:?}");
    println!("{g}");
    let l = List::empty();
    println!("{l}");
    let m = l.cons(List::lit("hallo"));
    println!("{m:?}");
    println!("{m}");
    let k = m.car();
    println!("{k:?}");
    println!("{k}");
    let o = List(g);
}
