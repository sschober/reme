mod reme;
use crate::reme::{cons, empty_list, lit};
/// dummy main that show cases how to construct and use a list
fn main() {
    let g = lit_list!("Hello", "World", "!");
    println!("{g}")
}
