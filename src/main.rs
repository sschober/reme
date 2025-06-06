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
    let m = List::lit("hallo").cons(l);
    println!("{m:?}");
    println!("{m}");
    let k = m.car();
    println!("{k:?}");
    println!("{k}");
    let n = m.cdr();
    println!("{n:?}");
    println!("{n}");
    let o = n.append(m);
    println!("{o:?}");
    println!("{o}");
    let p = o.append(List::lit("Welt").cons(List::empty()));
    println!("{p:?}");
    println!("{p}");
}
