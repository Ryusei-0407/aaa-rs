mod ecc;

use primitive_types::U512;

fn main() {
    println!("OK");
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);

    println!("{prime}");
}
