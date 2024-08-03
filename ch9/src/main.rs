fn main() {
    // closure inside `.filter_map()` needs to return an `Option`

    // there is a method called .ok() that turns Result into Option

    // .ok_or() and .ok_or_else() return an Option into a Result

    // .find() : I'll try to get it for you
    // .position() : I'll try to find there it is for you

    // .cycle() : create an iterator that loops forever
    // .zip() : create something new

    // .take_while() : takes into an iterator as long as it gets `true`
    // .take_while(|x| x < &5);
    //
    // .cloned() : makes a clone inside the iterator. this turns a reference into a value
    // .sum() : adds everything together
    // .by_ref() : makes an iterator take by reference

    // example 1 : .filter()
    let months = vec!["January", "February", "March", "April", "May",
                      "June", "July", "August", "September", "October", "November",
                      "December"];
    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect::<Vec<&str>>();
    println!("{:?}", filtered_months);

    // example 2 : .filter_map()
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let all_the_ceos = company_vec
        .iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();
    println!("{:?}", all_the_ceos);

    // example 3 : .filter_map()
    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];
    let successful_numbers = user_input
        .iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
    println!("{:?}", successful_numbers);

    // example 4 : .ok_or()
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let results: Vec<Result<String, &str>> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or("No CEO found"))
        .collect();
    for item in results {
        println!("{:?}", item);
    }

    // example 5 : .ok_or_else()
    println!("--- example 5 ---");
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let results: Vec<Result<String, String>> = company_vec
        .iter()
        .map(|company| {
            company.get_ceo().ok_or_else(|| {
                let err_message = format!("No CEO found for {}", company.name);
                println!("{err_message} at {}", get_current_datetime());
                err_message
            })
        })
        .collect();
    results
        .iter()
        .filter(|res| res.is_ok())
        .for_each(|res| println!("{res:?}"));

    // example 6 : .map() and .and_then()
    println!("--- example 6 ---");
    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];
    for index in 0..5 {
        char_vec.push(
            num_array
                .get(index)
                .and_then(|number| number.parse::<u32>().ok())
                .and_then(|number| char::try_from(number).ok()),
        );
    }
    println!("{:?}", char_vec);

    // example 7 : .and()
    println!("--- example 7 ---");
    let try_1 = [Some("Okay!"), None, Some("Okay!"), Some("Okay!"), None];
    let try_2 = [None, Some("Okay!"), Some("Okay!"), Some("Okay!"), Some("Okay!")];
    let try_3 = [Some("Okay!"), Some("Okay!"), Some("Okay!"), Some("Okay!"), None];
    for i in 0..try_1.len() {
        println!("{:?}", try_1[i].and(try_2[i]).and(try_3[i]));
    }

    // example 8 : .flatten() : ignore all `None` or `Err`
    println!("--- example 8 ---");
    for num in ["9", "nine", "ninety-nine", "9.9"]
        .into_iter()
        .map(|num| num.parse::<f32>())
        .flatten()
    {
        println!("{num:?}");
    }

    // example 9 : .any() and .all()
    println!("--- example 9 ---");
    let char_vec = ('a'..'働').collect::<Vec<char>>();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');

    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|&x| x.is_alphabetic())
    );
    println!(
        "All less than the character 행? {}",
        smaller_vec.iter().all(|&x| x < '행')
    );

    // example 10 : .cycle() and .zip()
    println!("--- example 10 ---");
    let even_odd_iter = ["even", "odd"].into_iter().cycle();
    let even_odd_vec: Vec<(i32, &str)> = (0..=5)
        .zip(even_odd_iter)
        .collect();
    println!("{:?}", even_odd_vec);

    // example 11 : .fold()
    println!("--- example 11 ---");
    let some_numbers = vec![9, 6, 9, 10, 11];
    println!("{}", some_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number));

    // example 12 : .fold()
    println!("--- example 12 ---");
    let events = [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ];
    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![],
    };
    let combined_events =
        events
            .iter()
            .fold(empty_events, |mut total_events, next_event| {
                total_events.num_of_events += 1;
                total_events.data.push(next_event.to_string());
                total_events
            });
    println!("{combined_events:#?}");

    // example 13 : .chunks() and .windows()
    println!("--- example 13 ---");
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];
    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }

    // example 14 : .match_indices()
    println!("--- example 14 ---");
    let some_str = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaß
    dich drauf.";
    for (index, item) in some_str.match_indices(|c| c > 'z') {
        println!("{item} at {index}");
    }
    for (index, item) in some_str.match_indices(". ") {
        println!("'{item}', at index {index}");
    }

    // example 15 : .peekable()
    println!("--- example 15 ---");
    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable();

    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        println!("{} is such a nice number", number_iter.peek().unwrap());
        number_iter.next();
    }

    // example 16 : dbg!
    println!("--- example 16 ---");
    let my_number = 8;
    dbg!(my_number);

    let mut my_number = dbg!(9);
    dbg!(my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);

    // example 17 : .inspect()
    println!("--- example 17 ---");
    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| println!("The item is: {first_item}"))
        .map(|x| x * 2)
        .inspect(|next_item| println!("Then it is: {next_item}"))
        .collect::<Vec<i32>>();

    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            println!("The item is: {first_item}");
            match **first_item % 2 {
                0 => println!("It is even."),
                _ => println!("It is odd."),
            }
            println!("In binary it is {:b}.", first_item);
        })
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
}

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn get_current_datetime() -> String {
    "2024-01-27T23:11:23".to_string()
}

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!(
        "Is {check} inside? {}",
        char_vec.iter().any(|&c| c == check)
    );
}

#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}
