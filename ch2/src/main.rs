const NUMBER_OF_MONTHS: u32 = 12;
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

fn main() {
    // reference : a memory-safe pointer
    // - A reference means you borrow the value, but you don’t own it.
    // - In Rust, reference has a `&` in front of them

    // &str (string slice) : a pointer to the data plus the length
    // - It might just be a partial view of the data owned by some other variable, so just a slice of it.

    // String : a pointer with data on the heap
    // - String owns its data, while a &str is a slice (a view into some data).
    // - A String is easy to grow, shrink, mutate, and so on.

    // str : a dynamically sized type
    // - Dynamically sized means that the size can be different.

    // String::From("string slice") : it is easy to make a String from a &str

    // std::mem::size_of::<String>();
    // std::mem::size_of::<i8>();
    // std::mem::size_of_val("test");

    // make a string
    // - String::from("shou") : method for String
    // - "shou".to_string() : method for &str
    // - format!("{}", "shou") : macro
    // - let my_string: String = "shou".into() : convert

    // Global variables last forever, so you don’t need to think about ownership for them

    // const & static
    // - no need `let` to declare

    // const is for values that don’t change and are created at compile time.
    // static is similar to const but has a fixed memory location. It might not be created at compile time.

    // & : referencing
    // * : dereferencing

    // Rust's reference rules
    // 1 (immutable references) : you can have as many immutable references as you want
    // 2 (mutable references) : you can only have one mutable reference
    // - you can't have an immutable reference and a mutable reference together

    // Rust’s simplest types are known as `Copy` types. They are all on the stack, and the compiler knows their size.

    // the compiler said that the data for String moved because a String isn’t a Copy type.
    // If it was a Copy type, the data would be copied, not moved.

    // if you can use an immutable reference, go with that.

    // a variable without a value is called an uninitialized variable
    // they can be useful when :
    // 1. you have a code block, and the value for your variable is inside it
    // 2. the variable needs to live outside the code block
    // 3. you want people reading you code to notice the variable name before the block

    // String for an owned type
    // &str for a borrowed string

    // more on references
    let country = String::from("Austria");
    let ref_one = &country;
    let ref_two = &country;
    println!("{}", ref_one);

    // mutable references
    let mut my_number = 8;
    // mutable reference to an i32
    // or ref mut i32
    let num_ref = &mut my_number;
    *num_ref += 10;
    println!("{}", num_ref);

    // mut reference
    let mut country = String::from("Austria");
    add_hungary(&mut country);

    // not using reference
    let country_1 = String::from("Austria");
    add_hungary_1(country_1);

    // clone string
    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length(&my_string);
    }

    // print!()
    print!("\\tThis is Shou\n\tNice to meet you.\n");
    println!("This is Shou
Nice to meet you
hahaha");

    // ignore escape with r#""#
    println!(r#"This is \n "Shou""#);
    println!(r####"#This is \n "Shou"#"####);

    // print bytes : b""
    println!("{:?}", b"This will look like numbers");

    // combine b and r
    println!("{:?}", br##"I like to write "#"."##);

    // unicode escape by \u{}
    println!("{:X}", '행' as u32);
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");

    // print memory : {:p}
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    // print binary : {:b}
    // print octal : {:o}
    // print hexadecimal : {:x}
    println!("binary: {:b}", number);
    println!("octal: {:o}", number);
    println!("hexadecimal: {:x}", number);

    // print with index
    println!("{1}, {2}, {0}", "a", "b", "c");
    println!("{b}, {c}, {a}", a = "a", b = "b", c = "c");

    // very complex printing
    // format: {variable:padding alignment minimum.maximum}
    // example: println!("{my_type:?}");
    let letter = "a";
    println!("{:ㅎ^11}", letter);

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}

fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it says: {country_name}");
}

fn add_hungary_1(mut string_to_add_hungary_to: String) {
    string_to_add_hungary_to.push_str("-Hungary");
    println!("Now it says: {string_to_add_hungary_to}");
}

fn get_length(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}
