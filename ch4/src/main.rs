use std::mem::size_of_val;

fn main() {
    // 3 types of structs
    // 1. unit struct
    // 2. tuple struct (unnamed struct)
    // 3. named struct

    // unit struct
    struct FileDirectory;

    // tuple struct
    struct ColorRgb(u8, u8, u8);

    // named struct
    struct SizeAndColor {
        size: u32,
        color: ColorRgb,
    }

    let my_color = ColorRgb(50, 0, 50);
    let size_and_color = SizeAndColor {
        size: 150,
        color: my_color,
    };

    // use struct when you want one thing `and` another thing
    // use enum when you want one thing `or` another thing

    // if an enum doesn't contain any data, then its variants can be cast into an integer

    // implementing : make functions that are part of the structs and enums themselves

    // 2 kinds of methods in an `impl` block
    // - methods : take `&self`, `&mut self` or `self`
    // - associated functions (static methods): do not take `self`

    // #[] : attributes
    // #[derive(Debug)]

    // when you use a method, Rust will dereference for you until it reaches the original type
    // when you use the dot operator, you don’t need to worry about *


    // example 1
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");
    let kalmykia = Country {
        population, // population: population,
        capital, // capital: capital,
        leader_name, // leader_name: leader_name,
        climate: Climate::Continental,
    }; // if the variable name equals to field name, then use it directly


    // example 2
    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);

    // example 3
    let my_mood = Mood::Happy;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {happiness_level}");

    // example 4 : use
    let size_of_jaurim = size_of_val("Shou");
    println!("{size_of_jaurim}");

    // example 5 : auto index
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }

    // example 6 : assigned value
    use Star::*;
    let five_stars = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in five_stars {
        println!("{}", star as u32);
    }

    // example 7 : put enum in vec
    let my_vec = vec![get_number(-800), get_number(8)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("A u32 with the value {number}"),
            Number::I32(number) => println!("An i32 with the value {number}"),
        }
    }

    // associated functions
    let mut my_string = String::from("I feel excited");
    my_string.push('!');

    // example 8 : implement methods
    let mut new_animal = Animal::new_cat();
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();

    // example 9 : destructuring
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let Person {
        name: fake_name,
        real_name,
        height: cm,
        happiness,
    } = papa_doc;

    println!("they call him {fake_name} but his real name is {real_name}.
    He is {cm} cm tall and is he happy? {happiness}");

    // example 10
    let tallinn = City::new("Tallinn", "Reval", 426_538, 1219);
    tallinn.print_names();

    // example 11
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    check_if_happy(&papa_doc);
    check_if_happy_destructured(&papa_doc);

    // example 12
    // dereferencing
    let my_name = "Shou".to_string();
    let my_ref = &my_name;
    println!("{}", &&&&my_name.is_empty());
}

enum Climate {
    Tropical,
    Dry,
    Temperate,
    Continental,
    Polar,
}

struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    climate: Climate,
}

enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")),
        _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{description}"),
        ThingsInTheSky::Stars(n) => println!("{n}"),
    }
}

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    let happiness_level = match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };
    happiness_level
}

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

enum Star {
    BrownDwarf,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };
    number
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new_cat() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("the animal is a dog"),
            AnimalType::Cat => println!("the animal is a cat"),
        }
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("changed animal to dog! now it's {self:?}");
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("changed animal to cat! now it's {self:?}");
    }
}

impl Mood {
    fn check(&self) {
        match self {
            Mood::Happy => println!("happy"),
            Mood::Angry => println!("angry"),
            Mood::NotBad => println!("not bad"),
            Mood::Sleepy => println!("sleepy"),
        }
    }
}

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

fn check_if_happy(person: &Person) {
    println!("Is {} happy? {}", person.name, person.happiness);
}

fn check_if_happy_destructured(Person { name, happiness, .. }: &Person) {
    println!("Is {name} happy? {happiness}");
}

struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(
        name: &str,
        name_before: &str,
        population: u32,
        date_founded: u32,
    ) -> Self {
        Self {
            name: String::from(name),
            name_before: String::from(name_before),
            population,
            date_founded,
        }
    }
    fn print_names(&self) {
        let City {
            name,
            name_before,
            ..
        } = self;
        println!("the city {name} used to be called {name_before}");
    }
}
