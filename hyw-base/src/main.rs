use hyw_base::{Hyw, HE, YI, WEI};

fn main() {
    println!("HE: {} chars", HE.len());
    println!("YI: {} chars", YI.len());
    println!("WEI: {} chars", WEI.len());
    println!("Total: {} chars", Hyw::all().size_hint().0);
}
