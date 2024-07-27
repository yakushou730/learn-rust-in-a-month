use std::fmt;
use std::fmt::{Debug, Formatter};

fn main() {
    // write a default method inside the trait if most users will use the methods the same way every time
    //
    // write the signature in trait if you think users will use the methods differently every time

    // if you implement Display, it automatically implements ToString
    // - ToString uses a blanket implementation, which means that it implements itself on any type that has Display

    // trait bounds : necessary traits in signature

    // From trait : you can make many types from many other types

    // the orphan rule
    // - you can implement your trait on someone else's type
    // - You can implement someone else's trait on your type
    // - However, you can't implement someone else's trait on someone else's type

    // newtype idiom : to wrap someone else's type in a tuple struct, thereby creating an entirely new type

    // AsRef trait : used to give a reference from one type to another type
    // used to do a cheap reference-to-reference conversion
    // both `String` and `str` implement AsRef<str>
    // taking an AsRef<str> is a convenient way to take both a `String` and a `&str` in a function


    // let rover = Dog {
    //     name: "Rover".to_string(),
    // };
    // let brian = Parrot {
    //     name: "Brian".to_string(),
    // };
    // rover.bark();
    // rover.run();
    // brian.bark();
    // brian.run();

    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();

    // implementing else's trait for our own type
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    println!("Mr. Mantle is a {mr_mantle:?}");
    println!("{mr_mantle}");

    print_excitedly(mr_mantle.to_string());
    println!("Mr. Mantle's String is {} letters long.",
             mr_mantle.to_string().chars().count());

    // example1 : our own struct
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };
    let mut uruk_hai = Monster { health: 40 };

    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);

    // example 2 : From<[T; N]>
    // making a `vec` from `[T; N]`, the const generics for an array
    // T : type, N : number
    let array_vec = Vec::from([8, 9, 10]);
    println!("Vec from array: {array_vec:?}");
    let str_vec = Vec::from("What kind of Vec am I?");
    println!("Vec from str: {str_vec:?}");
    let string_vec = Vec::from("What will a String be?".to_string());
    println!("Vec from String: {string_vec:?}");

    // example 3 : From trait
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let finland = Country::from(finland_cities);

    finland.print_cities();

    // newtype idiom
    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
    println!("{}", my_file.0 == my_string);

    // custom type with traits
    let file = File(String::from("I am file contents"));
    println!("{file:?}");
    println!("{file}");

    //
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}

// struct Dog {
//     name: String,
// }
//
// struct Parrot {
//     name: String,
// }

struct Animal {
    name: String,
}

// trait DogLike {
//     fn bark(&self) {
//         println!("Woof woof!");
//     }
//     fn run(&self) {
//         println!("The dog is running!");
//     }
// }

trait DogLike {
    fn bark(&self);
    fn run(&self);
}

// implement trait for concrete struct
// impl DogLike for Dog {}
// impl DogLike for Parrot {
//     fn run(&self) {
//         println!("{} the parrot is running", self.name);
//     }
// }

impl DogLike for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}

fn print_excitedly(input: String) {
    println!("{input} !!!!!");
}

#[derive(Debug)]
struct Monster {
    health: i32,
}

impl MonsterBehavior for Monster {
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait DisplayHealth {
    fn health(&self) -> i32;
}

trait FightClose {
    fn attack_with_sword<T: MonsterBehavior>(&self, opponent: &mut T) {
        println!("You attack with your sword!");
        opponent.take_damage(10);
        opponent.display_self();
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        println!("You attack with your hand!");
        opponent.take_damage(2);
        opponent.display_self();
    }
}

impl FightClose for Wizard {}

impl FightClose for Ranger {}

trait FightFromDistance: Debug {
    fn attack_with_bow<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
        println!("You attack with your bow!");
        if distance < 10 {
            opponent.take_damage(10);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }
    fn attack_with_rock<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
        println!("You attack with a rock!");
        if distance < 3 {
            opponent.take_damage(4);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }
}

impl FightFromDistance for Ranger {}

// to implement this trait, a type needs to have Debug
trait MonsterBehavior: Debug {
    fn take_damage(&mut self, damage: i32);
    fn display_self(&self) {
        println!("The monster is now: {self:?}");
    }
}

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!(
                "{:?} has a population of {:?}.",
                city.name, city.population,
            )
        }
    }
}

#[derive(Clone, Debug)]
struct File(String);

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let as_bytes = format!("{:?}", self.0.as_bytes());
        write!(f, "{as_bytes}")
    }
}

// AsRef trait has as_ref() method, it given a &str, and &str implements Display trait
fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref());
}
