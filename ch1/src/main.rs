/// this is learn-rust-in-a-month1 doc
fn main() {
    let mut my_name: String = "Dave".to_string();
    my_name.push_str("!");
    println!("{}, {my_name}", my_name);

    // std::mem::size_of::<char>() : byte size of a characters
    // len() : calculate bytes count
    // chars().count() : calculate the characters count
    // as_bytes() : change string to byte array

    // default integer type : i32
    // default float type : f64

    // cast : my_other_float as f64

    // -> : skinny arrow

    // println!("{}", variable) : Display printing
    // println!("{:?}", variable) : Debug printing
    // println!("{:#?}", variable) : pretty printing

    // trait : what a type can do

    // i8::MIN : min number of i8
    // i32::MAX : max number of i32

    // const (unchangeable global values) : in all capitals

    // mut : keyword for declare mutable variables

    // shadowing : useful in temp variables (like variables during math calculation)
    // - working quickly with variables we don’t care too much about
    // - getting around Rust’s strict rules about types, mutability, and so on
}
