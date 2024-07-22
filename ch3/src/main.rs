fn main() {
    // arrays : simple, fast, immutable collections of the same type
    // vectors : similar to arrays but growable and with more functionality
    // tuples : a grouping of various types

    // type of array : [type; number]
    // ex: [&str; 4]

    // b"&str" : transfer &str to byte array = an array of u8

    // vec : vector type
    // like String, it's owned types and very flexible

    // declare the item in vec
    // Vec<String>
    // Vec<(i32, i32)>

    // vec has capacity
    // if you go past the capacity, it will double its capacity and copy the items into this new memory space.
    // called -> reallocation

    // unit type : empty tuple

    // && : and
    // || : or

    // match :
    // use => (fat arrow) to say what to do when the pattern matches
    // each line called an `arm`
    // put a `comma` between the arms
    // each arm of a `match` has to return the same type

    // `..` : creates an exclusive range - `0..3` gives you `0, 1, 2`
    // `..=` : creates an inclusive range - `0..=3` gives you `0, 1, 2, 3`

    // _ : I don't care about this variable at all
    // _name : Maybe I will use it later

    let my_array = ["a"; 5];
    println!("{:?}", my_array);

    // get slice of an array
    // needs `&` : because compiler doesn't know the size of slice
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    println!("two_to_five: {:?}
start_at_one: {:?}
end_at_five: {:?}
everything: {:?}", two_to_five, start_at_one, end_at_five, everything);

    // Vec::new()
    let mut my_vec = Vec::new();
    my_vec.push("test");

    // declare type
    let mut my_vec: Vec<String> = Vec::new();
    let mut my_vec_2 = vec![8, 10, 10];
    let start_at_one = &my_vec_2[..1];
    println!("{:?}", start_at_one);

    // get capacity
    my_vec.push("shou".to_string());
    my_vec.push("anna".to_string());
    my_vec.push("foo".to_string());
    my_vec.push("bar".to_string());
    println!("{}", my_vec.capacity());
    my_vec.push("ai".to_string());
    println!("{}", my_vec.capacity());

    // initialize with capacity
    let mut num_vec = Vec::with_capacity(8);
    num_vec.push('a');

    // into()
    // make array to vec
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [1, 2, 3].into();

    // tuple
    // tuples are more like objects than indexed collections
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}\
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );

    // destructuring :
    // use a tuple to create multiple variables at the same time
    //
    // a String is not a copy type,
    // so the values are moved into a, b, and c,
    // and strings can't be accessed anymore
    let strings = ("one".to_string(), "two".to_string(), "three".to_string());
    let (_, b, c) = strings;
    println!("{}, {}", b, c);

    // if, else if, else
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six");
    } else {
        println!("It's a different number");
    }

    // match
    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("it's some other number"),
    }

    // declare a value with a match
    let my_number = 5;
    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };

    // match with tuple
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("it's dark and unpleasant today"),
        ("clear", "warm") => println!("it's a nice day"),
        ("cloudy", "warm") => println!("it's dark but not bad"),
        _ => println!("not sure what the weather is"),
    }

    // match guard
    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if !married => println!("not married with {children} kids"),
        (children, married) if children == 0 && married => {
            println!("married but no children")
        }
        _ => println!("married? {married}. number of children: {children}"),
    }

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {counter}");
        if counter == 5 {
            break;
        }
    }

    // give a loop a name
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");

    'first_loop: loop {
        counter += 1;
        println!("the counter is now: {}", counter);
        if counter > 5 {
            println!("Now entering the second loop.");

            'second_loop: loop {
                println!("the second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }

    // while loop
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("the counter is now: {counter}");
    }

    // for loop
    for number in 0..3 {
        println!("the number is : {}", number);
    }
    for number in 0..=3 {
        println!("the number is : {}", number);
    }
    for _ in 0..3 {
        println!("printing the same thing three times");
    }

    // break with value
    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("{my_number}");
}

fn match_colors(rgb: (i32, i32, i32)) {
    // a match statement always stops when if finds a match and doesn't check the rest
    match rgb {
        (r, _, _) if r < 10 => println!("not much red"),
        (_, g, _) if g < 10 => println!("not much green"),
        (_, _, b) if b < 10 => println!("not much blue"),
        _ => println!("each color has at least 10"),
    }
}

fn match_colors_better_version(rgb: (i32, i32, i32)) {
    let (red, green, blue) = (rgb.0, rgb.1, rgb.2);
    println!("comparing a color with {red} red, {blue} blue, and {green} green:");
    let color_vec = vec![(red, "red"), (blue, "blue"), (green, "green")];
    let mut all_have_at_least_10 = true;

    for (amount, color) in color_vec {
        if amount < 10 {
            all_have_at_least_10 = false;
            println!("not much {color}");
        }
    }
    if all_have_at_least_10 {
        println!("each color has at least 10");
    }
    println!();
}

fn match_number(input: i32) {
    match input {
        number @ 4 => println!("{number} is unlucky in China"),
        number @ 13 => println!("{number} is lucky in Italy"),
        number @ 14..=19 => println!("Some other number that ends with --teen: {number}"),
        _ => println!("some other number, I guess"),
    }
}
