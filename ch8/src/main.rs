use std::collections::HashMap;

fn main() {
    // method chaining : chain of methods

    // `for` loop gives you an iterator
    // - iter() : for an iterator of references
    // - iter_mut() : for an iterator of mutable references
    // - into_iter() : for an iterator of values (not references)

    // `for item in iterator` is as same as `for item in iterator.into_iter()`

    // `.map()` lets you do something to every item (including turning it into a different type) and then pass it on to make a new iterator
    // `.for_each()` lets you do something with every item without creating a new iterator

    // `.iter_mut()` plus `.for_each()` is basically a `for` loop
    // inside each method, we can give a name to every item (we called it `x`) and use that to change it.
    // These are called `closures`, so |x| means "`x` gets passed into the closure (the function)"

    // trait Iterator
    // - required associated types : Item
    // - required methods : next()

    // closures are quick functions that don't need a name - in other words, anonymous functions
    // sometimes they are called `lambdas` in other languages

    // closures can take variables from their environment that are outside the closure, even if you only write `||`
    // you can think of a closure as a standalone type that can hold references in the same way that a struct can

    // anonymous function : a `||` that doesn't enclose a variable from outside
    // closure : a `||` encloses a variable from outside

    // closures : lazy and fast
    // - `.map()` is that it doesn't do anything unless you use a method like `.collect()`

    // iterators are lazy and do nothing unless consumed

    // zero-cost abstractions
    // - complicated code might or might not take longer to compile,
    //   but at run time, tey will be the same speed


    // nonfunctional style : imperative style
    // imperative means to give orders or instructions
    let mut new_vec = Vec::new();
    let mut counter = 1;
    loop {
        new_vec.push(counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("{new_vec:?}");

    // functional style is more about expressions
    // taking the output of an expression, putting that into a new function,
    // taking that output, putting it into yet another function,
    // and so on until finally you have the result that you want
    let new_vec = (1..).take(10).collect::<Vec<i32>>();
    println!("{new_vec:?}");

    // with functional style, you can chain as many methods as you want.

    // example 1 : method chaining
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec
        .into_iter()
        .skip(3)
        .take(4)
        .collect::<Vec<i32>>();
    println!("{new_vec:?}");

    // example 2 : many types of iteration
    let vector1 = vec![1, 2, 3];
    let mut vector2 = vec![10, 10, 10];

    for num in vector1.iter() {
        println!("Printing a &i32: {num}");
    }
    for num in vector1 {
        println!("Printing an i32: {num}");
    }
    for num in vector2.iter_mut() {
        *num *= 10;
        println!("num is now {num}");
    }

    println!("{vector2:?}");
    // this line would break the code : `into_iter` takes ownership of the receiver `self`, which moves `vector1`
    // println!("{vector1:?}");

    // example 3 : another way for example 2
    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1
        .iter()
        .map(|x| x + 1)
        .collect::<Vec<i32>>();
    let vector1_b = vector1
        .into_iter()
        .map(|x| x + 10)
        .collect::<Vec<i32>>();
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x += 100);

    println!("{:?}", vector1_a);
    println!("{:?}", vector1_b);
    println!("{:?}", vector2);

    // example 4 : the `.next()` method in iterator
    let my_vec = vec!['a', 'b', '거', '柳'];

    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'柳'));
    assert_eq!(my_vec_iter.next(), None);
    assert_eq!(my_vec_iter.next(), None);

    // example 5 : implementing `Iterator` for our own types
    println!("example 5");
    let mut my_library = Library::new("Calgary");
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    for item in my_library.get_books() {
        println!("{item}");
    }

    // example 6 : use `.take()` method that only call specific times
    let five_ones: Vec<i32> = GivesOne.into_iter().take(5).collect();
    println!("{five_ones:?}");

    // bind closure to a variable
    let my_closure = |x: i32| println!("This is a closure: {x}");
    my_closure(1234);

    // for longer closures, you need to add a code block
    let my_closure = || {
        let number = 7;
        let other_number = 10;
        println!("The two numbers are {number} and {other_number}.");
    };
    my_closure();

    // example 7
    let number_one = 6;
    let number_two = 10;
    let my_closure = || println!("{}", number_one + number_two);
    my_closure();

    // example 8
    (1..=5).for_each(|num| {
        println!("Got a {num}!");
        if num % 2 == 0 {
            println!("it's even");
        } else {
            println!("it's odd");
        };
    });

    // example 9
    let nothing: Option<i32> = None;
    println!("{}", nothing.unwrap_or(0));

    // example 10
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if let Some(val) = my_vec.get(2) {
            val
        } else {
            &0
        }
    });
    println!("{fourth}");

    // example 11 : see the index of each item along with the item itself
    // zip with index in other language
    let char_vec = vec!['z', 'y', 'x'];
    char_vec
        .iter()
        .enumerate()
        .for_each(|(index, c)| println!("Index {index} is: {c}"));

    // example 12 : .map().collect()
    let num_vec = vec![2, 4, 6];
    let double_vec: Vec<i32> = num_vec
        .iter()
        .map(|num| num * 2)
        .collect();
    println!("{:?}", double_vec);

    // example 13 : use `.collect()` to make the `HashMap`
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    // let number_word_hashmap = some_keys
    //     .into_iter()
    //     .zip(some_values.into_iter())
    //     .collect::<HashMap<_, _>>();

    let number_word_hashmap: HashMap<_, _> = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect();

    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );

    // example 14 : .char_indices()
    let numbers_together = "140399923481800622623218009598281";
    for (index, num) in numbers_together.char_indices() {
        match (index % 3, num) {
            (0 | 1, num) => print!("{num}"),
            _ => print!("{num}\t"),
        }
    }

    // example 15 : |_|
    let my_vec = vec![8, 9, 10];
    my_vec
        .iter()
        .for_each(|_| println!("We didn't use the variables at all"));
}

#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Debug, Clone)]
struct BookCollection(Vec<String>);

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }
    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book: {book}");
                Some(book)
            }
            None => {
                println!("Out of books at the library!");
                None
            }
        }
    }
}

// this example implements the iterator that just gives the number 1 forever
struct GivesOne;

impl Iterator for GivesOne {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}
