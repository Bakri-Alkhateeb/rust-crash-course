#![deny(clippy::all)]

fn main() {
    let tuple = (25, "Bakri");

    let (age, name) = tuple;

    println!("{age} {name}");
}
