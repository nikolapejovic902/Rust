use std::io;

fn main() {
    println!("Unesite dužinu prve stranice: ");
    let mut stranica1 = String::new();
    io::stdin().read_line(&mut stranica1).expect("Greška pri unosu.");
    let stranica1: f32 = stranica1.trim().parse().expect("Greška pri konverziji.");

    println!("Unesite dužinu druge stranice: ");
    let mut stranica2 = String::new();
    io::stdin().read_line(&mut stranica2).expect("Greška pri unosu.");
    let stranica2: f32 = stranica2.trim().parse().expect("Greška pri konverziji.");

    let obim = 2.0 * (stranica1 + stranica2);
    println!("Obim pravougaonika je: {}", obim);
}
