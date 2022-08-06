#![deny(clippy::all)]

fn main() {
    println!("============== Variables ==============");
    variables();
    println!("============== Ownership ==============");
    ownership();
}

fn ownership() {
    // ==========================================================
    // name2 takes the ownership of the pointer to the heap from name1
    // because we can't have two variables pointing to the same heap in memory
    // let name1 = String::from("Bakri");
    // let name2 = name1;

    // println!("Hello {name1}");
    // println!("Hello {name2}");

    // {
    //     let name = "Bakri".to_string();
    //     println!("Hello {name}");
    // }

    // let age1 = 10;
    // let age2 = age1;

    // println!("You are {age1} years old");
    // println!("You are {age2} years old");

    // ==========================================================
    // References solves the ownership problem
    // let name1 = String::from("Bakri");
    // let name2 = &name1;

    // greet(&name1);
    // greet(name2);

    // fn greet(name: &String) {
    //     println!("Hello {name}");
    // }

    // let mut name1 = String::from("Bakri");

    // clear_string(&mut name1);

    // ==========================================================
    // There can be at most one mutable reference of a variable

    // fn clear_string(value: &mut String) {
    //     value.clear();
    //     println!("Cleared Variable");
    // }

    // let mut name1 = String::from("Bakri");
    // let mut name2 = &mut name1;
    // let mut name3 = &mut name1;

    // clear_string(&mut name2);

    // ==========================================================
    // There can't be a mutable referenece while there is
    // already an immutable referenece to the variable

    // let mut name1 = String::from("Bakri");
    // let name2 = &name1;
    // let mut name3 = &mut name1;

    // println!("{name1}");
    // println!("{name2}");
    // println!("{name3}");

    // ==========================================================
    // Dangling references

    // fn get_name() -> &String {
    //     &"Bakri".to_string();
    // }

    // let name = get_name();
}

fn variables() {
    // let first_name = "Bakri";
    // let last_name = "Alkhateeb";

    // println!("My Full Name is {first_name} {last_name}");

    let tuple = (25, "Bakri");

    let (age, name) = tuple;

    println!("{age} {name}");
}
