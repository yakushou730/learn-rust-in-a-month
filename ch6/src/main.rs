use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, VecDeque};
use std::num::ParseIntError;

fn main() {
    // HashMap is a collection made out of keys and values
    // The keys of a HashMap are not ordered

    // BTreeMap is similar to HashMap but provide order (alphabetical)

    // the program will `crash` if you try to get a non-existed key in HashMap by []

    // if a HashMap already has a key when you try to put it in, using `.insert()`will overwrite its value

    // .entry(key) takes a key, then returns an enum called Entry
    // enum Entry<K, V> {
    //     Occupied(OccupiedEntry<K, V>),
    //     Vacant(VacantEntry<K, V>),
    // }

    // HashSet : implemented as a HashMap where the value is ()
    // BTreeSet : implemented as a BTreeMap where the value is (), has default order

    // BinaryHeap : it keeps the item with the greatest value in the front,
    // but the other items are in any order.
    // some languages call this a priority queue.

    // VecDeque : optimized for popping items both off the front and the back
    // it uses something called a ring buffer
    // (Vec is great for popping off the back, but not so great off the front)

    // ? (question mark operator)
    // after anything that returns a Result, you can add `?`
    // - give what is inside the `Result` if it is `Ok`
    // - pass the error back if it is Err (this is called an early return)

    // `panic!` is a good macro to create reminders in your code

    // assert! : if the part inside () is not true, the program will panic
    // assert_eq! : the two items inside () must be equal
    // assert_ne! : the two items inside () must not be equal

    // using [vec].get(i).expect("message") is better than using [vec].get(i).unwrap()
    // expect("message") provides more information


    // HashMap
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    for (year, population) in tallinn.population {
        println!("In {year}, Tallinn had a population of {population}.");
    }

    // BTreeMap
    let mut tallinn = City2 {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    for (year, population) in tallinn.population {
        println!("In {year}, Tallinn had a population of {population}.");
    }

    // try to get a non-existed key
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }
    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));

    // insert same key
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(&1)); // The .get() method takes a reference, which is why we have &1 here

    // prevent insert duplicated key
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");

    let key = 1;
    match book_hashmap.get(&key) {
        Some(val) => println!("Key {key} has a value already: {val}"),
        None => {
            // because insert() would return Option(old_existed_value),
            // so we need {} to ensure the return type is () as same as other arm
            book_hashmap.insert(key, "Le Petit Prince");
        }
    }

    // store old value to vec
    let mut book_hashmap = HashMap::new();
    let mut old_hashmap_values = Vec::new();

    let hashmap_entries = [
        (1, "L'Allemagne Moderne"),
        (1, "Le Petit Prince"),
        (1, "섀도우 오브 유어 스마일"),
        (1, "Eye of the World"),
    ];

    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap.insert(key, value) {
            println!("Overwriting {old_value} with {value}!");
            old_hashmap_values.push(old_value);
        }
    }
    println!("All old values: {old_hashmap_values:?}");

    // .entry(key) to make an entry and then using `or_insert()` to insert a default value
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        *return_value += 1;
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book}? {true_or_false}");
    }

    // .or_insert().push()
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();
    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }
    for (male_or_female, numbers) in survey_hash {
        println!("{male_or_female}: {numbers:?}");
    }

    // BTreeMap
    let many_numbers = vec![37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48,
                            28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21, 20, 38, 16, 48, 39, 31, 41,
                            32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9, 46, 33];

    let mut number_set = BTreeSet::new();
    for number in many_numbers {
        number_set.insert(number);
    }
    for number in number_set {
        println!("{number}");
    }

    // BinaryHeap
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }
    println!("First item is largest, others are out of order: {heap:?}");
    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }

    // another BinaryHeap case
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Reply to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    for (_, job) in jobs {
        println!("You need to: {job}");
    }

    // VecDeque
    let mut my_vec = VecDeque::from(vec![0; 10_000]);
    for i in 0..10_000 {
        my_vec.pop_front();
    }

    // question mark operator
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_and_log_str(item);
        println!("Result: {parsed:?}");
    }

    // multiple error types by using multiple ?
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }

    // panic!() macro
    // panic!("this is panic");

    // assert!, assert_eq!, assert_ne!
    let my_name = "Loki Laufeyson";

    assert!(my_name == "Loki Laufeyson");
    assert_eq!(my_name, "Loki Laufeyson");
    assert_ne!(my_name, "Mithridates");

    // unwrap_or()
    // get the default value if it gets None, never panic
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or(&0);
    println!("{fourth}");
}

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

struct City2 {
    name: String,
    population: BTreeMap<i32, i32>,
}

fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
    // if it is `Ok`, it gives an i32 wrapped in `Ok`
    // if it is `Err`, it returns a ParsIntError, and the function is over
    let parsed_number = input.parse::<i32>()?;

    // the ? does
    //
    // let parsed_number = match input.parse::<i32>() {
    //     Ok(number) => number,
    //     Err(e) => return Err(e),
    // };

    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}
