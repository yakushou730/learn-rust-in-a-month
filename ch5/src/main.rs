use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};

fn main() {
    // Generics : when to use more than one type
    //            maybe one type, maybe another type
    // Option : when an operation might produce a value but might not
    // Result : when an operation might succeed but might not

    // Panic means that the program stops before the problem happens
    // Rust sees that the function wants something impossible and stops
    // It "unwinds the stack" (takes the values off the stack) and tells you, "Sorry, I can’t do that."

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // enum Result<T, E> {
    //     OK(T),
    //     Err(E),
    // }

    // Option holds a Some or None
    // Result holds an OK or Err
    // Result<Option<SomeType>>

    // check the state
    // Option : .is_some(), .is_none()
    // Result : .is_ok(), is_err()

    // if you have a function that could panic,
    // try turning its output into an Option or a Result

    // generic
    let _item = return_item(5);

    // generic with trait
    print_item(5);

    // generic with custom type which derive Debug
    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };
    let number = 55;
    print_item(charlie);
    print_item(number);

    // more than one generic type
    compare_and_display("Listen up!", 9, 8);

    // Option
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    // If you unwrap a value that is None, the program will panic
    println!("{:?}, {:?}", try_take_fifth(small), try_take_fifth(big).unwrap());

    // Option - with match to handle
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new();

    option_vec.push(try_take_fifth(small));
    option_vec.push(try_take_fifth(big));

    handle_options(&option_vec);

    // Option - is_some(), is_none()
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    for vec in vec![small, big] {
        let inside_number = try_take_fifth(vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap());
        } else {
            println!("We got nothing");
        }
    }

    // Result
    check_error();

    // Result with Err check
    if see_if_number_is_even(5).is_ok() {
        println!("it's okay, guys");
    } else {
        println!("it's an error, guys");
    }

    // Result with more information of Err check
    for number in 4..=7 {
        println!("{:?}", check_if_five(number));
    }

    // IF LET : do something if it matches, and don't do anything if it doesn't
    let my_vec = vec![2, 3, 4];
    // let get_one = my_vec.get(0);
    // let get_two = my_vec.get(10);
    // println!("{:?}", get_one);
    // println!("{:?}", get_two);

    // for index in 0..10 {
    //     match my_vec.get(index) {
    //         Some(number) => println!("The number is: {number}"),
    //         None => {}
    //     }
    // }

    for index in 0..10 {
        // if you get the pattern Some(number) from my_vec.get(index)
        // use `=`, not `==` because it is a pattern match, not a boolean
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {number}");
        }

        let Some(number) = my_vec.get(index) else {
            // diverging code : continue, break, early return
            continue;
        };
        println!("The number is: {number}");
    }

    // WHILE LET : like a `while` loop for `if let`
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];
    for mut city in weather_vec {
        println!("For the city of {}:", city[0]);
        while let Some(information) = city.pop() {
            if let Ok(number) = information.parse::<i32>() {
                println!("The number is: {number}");
            }
        }
    }
}

// people would say :
// the function `return_item` is generic over type `T`
fn return_item<T>(item: T) -> T {
    println!("here is your item");
    item
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input_1: U, input_2: U) {
    println!("{statement}! Is {input_1} greater than {input_2}? {}", input_1 > input_2);
}

fn compare_and_display_by_where<T, U>(statement: T, input_1: U, input_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{statement}! Is {input_1} greater than {input_2}? {}", input_1 > input_2);
}

fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_options(my_option: &Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {number}!"),
            None => println!("Found a None!"),
        }
    }
}

fn check_error() -> Result<(), ()> {
    Ok(())
}

fn see_if_number_is_even(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err(format!("Sorry, bad number. Expected: 5 Got: {number}")),
    }
}
