mod reme;
use crate::reme::List;
/// dummy main that show cases how to construct and use a list
fn main() {
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
    let p = o.append(lit_list!("Welt"));
    println!("{p:?}");
    println!("{p}");
    let q = p.append(lit_list!("!"));
    println!("{q:?}");
    println!("{q}");
    let r = q.reverse();
    println!("{r:?}");
    println!("{r}");
    println!("r.length() = {}", r.length());
}
