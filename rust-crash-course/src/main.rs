#![deny(clippy::all)]

// use std::collections::{btree_map::ValuesMut, HashMap};

fn main() {
    println!("============== Variables ==============");
    variables();
    println!("============== Ownership ==============");
    ownership();
    println!("============== Functions ==============");
    functions();
    println!("============== Structures ==============");
    structures();
    println!("============== Enumerations ==============");
    enunmerations();
    println!("============== Collections ==============");
    collections();
    println!("============== Optionals ==============");
    optionals();
    println!("============== Error Handling ==============");
    error_handling();
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

    // let tuple = (25, "Bakri");

    // let (age, name) = tuple;

    // println!("{age} {name}");
}

fn functions() {
    // This function returns a Stirng, but we can say that this
    // function does not return anything by either saying
    // fn say_hello_world() -> () {} which means return unit type
    // or saying fn say_hello_world(){}
    // fn say_hello_world() -> String {
    //     // This is a shorthand for return statement
    //     // without a return and without a semicolon
    //     String::from("Hello, World!")
    // }

    // fn say_hello(to_person: String) -> String {
    //     format!("Hello {}!", to_person)
    // }

    // let message = say_hello_world();
    // println!("{message}");
    // let hello = say_hello(String::from("Bakri"));
    // println!("{hello}");

    // // Inline Function
    // let say_hello_to = |name: &str| format!("Hello, {}!", name);

    // let hello2 = say_hello_to("Bakri");

    // println!("{hello2}");

    // let ask_for_age = |age: i32| age + 10;

    // let age = ask_for_age(25);

    // println!("{age}");

    // fn process_name(name: &str, callback: fn(&str) -> ()) {
    //     callback(name);
    // }

    // process_name("Bakri", |name: &str| println!("{name}"));
}

fn structures() {
    // This is instead of a class which rust does not support any more
    // struct Person {
    //     name: String,
    //     age: u8,
    // }

    // fn create_person(name: String, age: u8) -> Person {
    //     Person { name, age }
    // }

    // let person = create_person(String::from("Bakri"), 25);

    // // Struct update index syntax
    // let person2 = Person {
    //     name: "John".to_string(),
    //     ..person
    // };

    // println!("{} is {} years old", person2.name, person2.age);
    // #[derive(Debug)]
    // struct Point(f64, f64, f64);

    // let point = Point(20.2, 5.0, 10.7);

    // // println!("X = {}, Y = {}, Z = {}", point.0, point.1, point.2);

    // impl Point {
    //     fn describe(&self) {
    //         println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
    //     }

    //     fn double_point(&self) -> Point {
    //         Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    //     }

    //     fn make_twice(&mut self) {
    //         self.0 *= 2.0;
    //         self.1 *= 2.0;
    //         self.2 *= 2.0;
    //     }

    //     fn zero() -> Point {
    //         Point(0.0, 0.0, 0.0)
    //     }
    // }

    // point.describe();
    // point.double_point().describe();

    // println!("{:?}", point);

    // let mut point3 = Point(1.0, 2.0, 3.0);
    // let point4 = point3.double_point();
    // point4.describe();
    // point3.make_twice();
    // point3.describe();

    // let point5 = Point::zero();
    // let point6 = Point::zero();
    // let point7 = Point::zero();

    // point5.describe();
    // point6.describe();
    // point7.describe();
}

fn enunmerations() {
    // #[derive(PartialEq)]
    // enum AnimalType {
    //     Dog,
    //     Cat,
    //     Rabbit,
    // }

    // let fluffy = AnimalType::Dog;

    // if fluffy == AnimalType::Dog {
    //     println!("Fluffy is a Dog");
    // } else {
    //     println!("Fluffy is not a Dog");
    // }

    // This is the switch statement in Rust
    // match fluffy {
    //     AnimalType::Dog => println!("Fluffy is a Dog"),
    //     AnimalType::Cat => println!("Fluffy is a Cat"),
    //     AnimalType::Rabbit => println!("Fluffy is a Rabbit"),
    //     // This is the default case
    //     _ => println!("Fluffy is Something Else"),
    // };

    // enum Shapes {
    //     Rectangle { width: f64, height: f64 },
    //     Circle { radius: f64, center: (f64, f64) },
    // }

    // let rect = Shapes::Rectangle {
    //     width: 10.0,
    //     height: 5.0,
    // };

    // let circ = Shapes::Circle {
    //     radius: 20.1,
    //     center: (0.0, 0.0),
    // };

    // if let Shapes::Rectangle { width, height } = rect {
    //     println!("Width is {}, Height is {}", width, height);
    // }

    // impl Shapes {
    //     fn area(&self) -> f64 {
    //         match self {
    //             Shapes::Rectangle { width, height } => width * height,
    //             Shapes::Circle { radius, center } => radius * radius * PI,
    //         }
    //     }
    // }

    // let rect_area = rect.area();
    // let circ_area = circ.area();

    // println!("Rectangle Area is {}", rect_area);
    // println!("Circle Area is {}", circ_area);

    // enum Pet {
    //     Cat { name: String },
    //     Dog { name: String },
    // }

    // let fluffy = Pet::Cat {
    //     name: "Fluffy".to_string(),
    // };

    // let name = match fluffy {
    //     Pet::Cat { name } => name,
    //     Pet::Dog { name } => name,
    // };

    // println!("Name is {}", name);
}

fn collections() {
    // Tuples
    // struct Point(f32, f32);
    // let values = ("Hello", "World", 30);
    // let hello = values.0;
    // let world = values.1;
    // let number = values.2;
    // let (_, _, age) = values;

    // fn get_values() -> (String, String, i32) {
    //     ("Hello".to_string(), "World".to_string(), 30)
    // }

    // let (hello, _, _) = get_values();

    // Vectors
    // let vectors: [&str; 2] = ["Foo", "bar"];
    // for value in vectors.iter() {
    //     println!("{}", value);
    // }
    // let foo = &vectors[0];
    // println!("{}", vectors.len());
    // let vectors2: [i32; 2] = [10, 20];
    // let doubled = vectors2.iter().map(|x| x * 2);
    // println!("{}", doubled[0]);
    // let mut vectors = vec![1, 2, 3, 4, 5, 6];
    // let mut values2 = vec![1, 2, 3, 4, 5, 6];
    // vectors.push(7);
    // println!("{:?}", vectors);
    // let seven = vectors.pop();
    // println!("{:?}", seven);
    // vectors.extend_from_slice(&[8, 9, 10]);
    // println!("{:?}", vectors);
    // vectors.append(&mut values2);
    // println!("{:?}", vectors);
    // println!("{:?}", values2);

    // if vectors.contains(&3) {
    //     println!("Yes");
    // } else {
    //     println!("No");
    // }

    // if values2.is_empty() {
    //     println!("Yes");
    // } else {
    //     println!("No");
    // }

    // HashMaps (needs to be imported using std::collections::HashMap)
    // let mut values: HashMap<&str, &str> = HashMap::new();
    // values.insert("foo", "Bar");

    // println!("{:?}", values);

    // values.remove("foo");
    // println!("{:?}", values);
    // values.insert("name", "Bar");
    // // unsafe reading
    // println!("{}", values["name"]);
    // // safe reading
    // match values.get("foo") {
    //     Some(value) => println!("{}", value),
    //     None => println!("Key (foo) does not exist"),
    // };

    // for (&key, &value) in &values {
    //     println!("Key ({}) has Value ({})", key, value);
    // }

    // values.entry("foo").or_insert("Jane");

    // for (&key, &value) in &values {
    //     println!("Key ({}) has Value ({})", key, value);
    // }

    // #[derive(Hash, Eq, PartialEq, Debug)]
    // struct Person {
    //     name: String,
    //     age: u8,
    // }

    // let mut people: HashMap<Person, &str> = HashMap::new();
    // let person = Person {
    //     name: "Bakri".to_string(),
    //     age: 25,
    // };
    // people.insert(person, "Bakri");

    // Iterators
    // let values = vec![1, 2, 3, 4, 5];

    // for value in values.iter() {
    //     println!("{}", value);
    // }

    // let iter = values.iter();

    // let sum: i32 = iter.sum();
    // // iterators are consumables
    // let iter2 = values.iter();
    // let sum2: i32 = iter2.sum();

    // println!("Sum is {}", sum);
    // println!("Sum2 is {}", sum2);

    // let doubled: Vec<i32> = values.iter().map(|x| x * 2).collect();
    // println!("Doubled is {:?}", doubled);
    // let odd: Vec<i32> = values.into_iter().filter(|x| x % 2 == 0).collect();
    // println!("ODD is {:?}", odd);
}

fn optionals() {
    // let value = Some(10);
    // let name: Option<&str> = None;

    // match name {
    //     Some(n) => println!("Hello {}", n),
    //     None => println!("No Name Provided"),
    // };

    // let unwrapped_name = name.expect("Name was not provided");
    // let unwrapped_name2 = name.unwrap();
    // println!("Name is {}", unwrapped_name);
    // println!("Name is {}", unwrapped_name2);
    // let mut age: Option<i8> = Some(20);

    // match age.as_mut() {
    //     Some(age) => *age += 10,
    //     None => {}
    // }

    // println!("Age is {:?}", age);
}

fn error_handling() {
    // Result<ValueType, ErrorType>
    // let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");

    // match value {
    //     Ok(v) => println!("Value is {}", v),
    //     Err(e) => println!("Error is {}", e),
    // }

    // let value2: Result<&str, ()> = Err(());

    // match value2 {
    //     Ok(v) => println!("Value is {}", v),
    //     Err(_) => println!("Some error occurred"),
    // }

    // fn get_user_name() -> Result<String, ()> {
    //     Ok("Bakri".to_string())
    //     // Err(())
    // }

    // let user_name = get_user_name().expect("I was expecting a Username");
    // println!("Username is {}", user_name);

    // fn get_user_name2() -> Result<String, ()> {
    //     // Ok("Bakri".to_string())
    //     Err(())
    // }

    // let user_name2 = get_user_name2().expect_err("I was expecting an Err");

    // fn get_first_name() -> Result<String, String> {
    //     // Ok("Bakri".to_string())
    //     Err("I don't know the first name".to_string())
    // }

    // fn get_last_name() -> Result<String, String> {
    //     Ok("Alkhateeb".to_string())
    // }

    // fn get_full_name() -> Result<String, String> {
    //     let first_name = get_first_name()?;
    //     let last_name = get_last_name()?;

    //     Ok(format!("{} {}", first_name, last_name))
    // }

    // let full_name = get_full_name();

    // match full_name {
    //     Ok(n) => println!("{}", n),
    //     Err(_) => {}
    // }

    // let length = full_name.map(|name| name.len()).unwrap_or_default();

    // println!("Length is {}", length);

    // let error_length = full_name.map_err(|err| err.len());
    // println!("Error Length is {:?}", error_length);
}
