use hyw_base::{Hyw, HE, YI, WEI};

fn main() {
    println!("HE: {} chars", HE.len());
    println!("YI: {} chars", YI.len());
    println!("WEI: {} chars", WEI.len());

    let size = Hyw::all().size_hint().0;
    println!("Total: {size} combinations");
    println!("Approx. embedding size: {}MB", 4 * size / 1024); // 4 bytes per f32, 1024 floats per embedding
}
