#![deny(clippy::all)]

fn main() {
    println!("============== Variables ==============");
    variables();
    println!("============== Ownership ==============");
    ownership();
    println!("============== Functions ==============");
    functions();
    println!("============== Structures ==============");
    structures();
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

fn functions() {
    // This function returns a Stirng, but we can say that this
    // function does not return anything by either saying
    // fn say_hello_world() -> () {} which means return unit type
    // or saying fn say_hello_world(){}
    fn say_hello_world() -> String {
        // This is a shorthand for return statement
        // without a return and without a semicolon
        String::from("Hello, World!")
    }

    fn say_hello(to_person: String) -> String {
        format!("Hello {}!", to_person)
    }

    let message = say_hello_world();
    println!("{message}");
    let hello = say_hello(String::from("Bakri"));
    println!("{hello}");

    // Inline Function
    let say_hello_to = |name: &str| format!("Hello, {}!", name);

    let hello2 = say_hello_to("Bakri");

    println!("{hello2}");

    let ask_for_age = |age: i32| age + 10;

    let age = ask_for_age(25);

    println!("{age}");

    fn process_name(name: &str, callback: fn(&str) -> ()) {
        callback(name);
    }

    process_name("Bakri", |name: &str| println!("{name}"));
}

fn structures() {
    // This is instead of a class which rust does not support any more
    struct Person {
        name: String,
        age: u8,
    }

    fn create_person(name: String, age: u8) -> Person {
        Person { name, age }
    }

    let person = create_person(String::from("Bakri"), 25);

    // Struct update index syntax
    let person2 = Person {
        name: "John".to_string(),
        ..person
    };

    println!("{} is {} years old", person2.name, person2.age);
    #[derive(Debug)]
    struct Point(f64, f64, f64);

    let point = Point(20.2, 5.0, 10.7);

    // println!("X = {}, Y = {}, Z = {}", point.0, point.1, point.2);

    impl Point {
        fn describe(&self) {
            println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
        }

        fn double_point(&self) -> Point {
            Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
        }

        fn make_twice(&mut self) {
            self.0 *= 2.0;
            self.1 *= 2.0;
            self.2 *= 2.0;
        }

        fn zero() -> Point {
            Point(0.0, 0.0, 0.0)
        }
    }

    point.describe();
    point.double_point().describe();

    println!("{:?}", point);

    let mut point3 = Point(1.0, 2.0, 3.0);
    let point4 = point3.double_point();
    point4.describe();
    point3.make_twice();
    point3.describe();

    let point5 = Point::zero();
    let point6 = Point::zero();
    let point7 = Point::zero();

    point5.describe();
    point6.describe();
    point7.describe();
}
