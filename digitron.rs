use std::io;

fn main() {
    println!("Odaberite operaciju koju želite da izvršite:");
    println!("1 - Sabiranje");
    println!("2 - Oduzimanje");
    println!("3 - Množenje");
    println!("4 - Deljenje");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Nije uspelo čitanje linije");

    let choice: i32 = choice.trim().parse().expect("Unos mora biti broj");

    println!("Unesite prvi broj:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Nije uspelo čitanje linije");

    println!("Unesite drugi broj:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Nije uspelo čitanje linije");

    let num1: f64 = num1.trim().parse().expect("Unos mora biti broj");
    let num2: f64 = num2.trim().parse().expect("Unos mora biti broj");

    let result = match choice {
        1 => num1 + num2,
        2 => num1 - num2,
        3 => num1 * num2,
        4 => num1 / num2,
        _ => {
            println!("Nepoznata opcija.");
            return;
        }
    };

    if choice == 4 {
        println!("Rezultat: {:.3}", result);
    } else {
        println!("Rezultat: {}", result);
    }
}
