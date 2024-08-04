use std::cell::{Cell, RefCell};
use std::sync::{Mutex, RwLock};

fn main() {
    // other types of &str
    // - String literals : `let my_str = "I am a &str"`
    //   they last for the whole program because they are written directly into the binary
    //   they have the type `&'static str`
    //   the `'` means it's lifetime, and string literals have a lifetime call `static`
    // - borrowed `str` :
    //   this is the regular `&str` from without a `'static` lifetime
    //   if you have a `String` and pass a reference to it (a `&String`), Rust will convert it to a `&str` when you need it
    //   this is thanks to a trait call `Deref`
    //
    // you can pass in a `&String` to a function that takes a `&str`

    // using a `str` with a `'static` lifetime, the data never disappears

    // lifetime : means `how log the variable or reference lives`

    // lifetime annotation : extra lifetime information
    // you only need to think about lifetimes with references

    // `missing lifetime specifier` means that we need to add a `'` to the lifetime

    // we cannot return a reference to it because it is going to die after the next line

    // `fn returns_str() -> &'static str` tells Rust : Don't worry; we will only return a string literal
    // String literals live for the whole program, so the compiler is now happy with it

    // `name: &'a str`
    // the City struct has a lifetime that we will call `'a`, and its `name` property must also live at least as long as `'a`
    // other shorter lifetimes will not be accepted
    //
    // usually, you will write `'a', 'b', 'c'`, etc., because it is quick and the usual way to write

    // `<'_>` called the `anonymous lifetime` and is an indicator that references are being used

    // impl <'a, 'b> HasSomeLifeTime<'a, 'b> for SomeStruct<'a, 'b> {}
    // - means "We are talking about two different lifetimes here, 'a and 'b"
    //   now, the 'a and 'b for the trait and the struct are the same lifetime

    // impl <'one, 'two, 'three, 'four> HasSomeLifeTime<'one, 'three> for SomeStruct<'two, 'four> {}
    // - means "There are four lifetimes involved here," and the trait has its own two while the struct has its own two
    //   the four lifetimes can now all be separate from each other
    // - but don't worry. we can ust elide the lifetimes and let Rust figure it out
    //
    // `impl HasSomeLifeTime<'_, '_> for SomeStruct<'_, '_> {}`
    // - means "each one has its own two lifetimes; you figure it out"

    // interior mutability
    // - having a little bit of mutability on the inside (the interior)

    // Rust has four main ways to allow some safe mutability inside of something that is immutable:
    // `Cell, RefCell, Mutex, and RwLock`

    // Cell : mutable memory location
    // - `Cell` works for all types, but it works best for simple `Copy` types because it gives values, not references
    // - `Cell` has a method called `.get()`

    // RefCell : reference cell
    // - `.borrow()` : &
    // - `.borrow_mut()` : &mut
    // - have to be careful with a `RefCell` because it checks borrows at run time, not compilation time
    //
    // - use the `.try_borrow_mut()` method instead of `borrow_mut()` if there is a chance of a double borrow
    //   this will return an error if the `RefCell` is already borrowed

    // Mutex : mutual exclusion, which means "only one at a time"
    // - only lets one thread change it at a time by using `.lock()`, which returns a struct called a `MutexGuard`
    //
    // - if another variable tries to `.lock()` it, it will wait forever, this is known as a deadlock
    //
    // - instead of `.lock()`, you can use a method called `.try_lock()`.
    //   this method will try once, and if it doesn't get the lock, it will give up

    // RwLock : read-write lock
    // - it's not only like a Mutex because it is thread-safe, but it is also similar to a `RefCell`
    //   you can get mutable or immutable references to the value inside
    //   RwLock is similar to RefCell because it follows the same rules that Rust uses for references
    //
    // - .write().unwrap() : to change
    // - .read().unwrap() : to get read access
    // - .try_read() and .try_write()

    // example 1 : borrowed `str`
    println!("--- example 1 ---");
    let my_string = String::from("I am a string");
    prints_str(&my_string);

    // example 3 : lifetime annotations
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City {
        name: &city_names[0],
        date_founded: 1932,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    // example 5 : cell
    println!("--- example 5 ---");
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };
    super_phone_3000.make_not_on_sale();
    println!("{super_phone_3000:#?}");

    // example 6 : RefCell
    println!("--- example 6 ---");
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{:?}", user_1.active);
    // let borrow = &mut user_1.active;
    // *borrow = false;
    let mut borrow = user_1.active.borrow_mut();
    *borrow = false;
    // *user_1.active.borrow_mut() = false;
    println!("{:?}", user_1.active);

    // example 7 : Mutex
    println!("--- example 7 ---");
    let my_mutex = Mutex::new(5);
    // or by `drop(mutex_changer)` to make an object go out of scope
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        println!("{my_mutex:?}");
        println!("{mutex_changer:?}");
        *mutex_changer = 6;
        // *my_mutex.lock().unwrap() = 6;
        println!("{mutex_changer:?}");
    }
    println!("{my_mutex:?}");

    // example 8 : .try_lock()
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();

    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {value}");
    } else {
        println!("Didn't get the lock");
    }

    // example 9 : directly lock without variable, so it unlocks immediately after lock
    let my_mutex = Mutex::new(5);
    for _ in 0..100 {
        *my_mutex.lock().unwrap() += 1;
    }

    // example 10 : avoid dead lock by drop()
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");
    drop(read1);
    drop(read2);

    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    drop(write1);
    println!("{:?}", my_rwlock);


    // example 11 : .try_write()
    println!("--- example 11 ---");
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    if let Ok(mut number) = my_rwlock.try_write() {
        *number += 10;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };
}

fn prints_str(my_str: &str) {
    println!("{my_str}");
}

// example 2 : this function works
fn works() -> &'static str {
    "I live forever!"
}

// the `'a` means "Please only take an input for `name` if it lives at least as long as `City`"
#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    date_founded: u32,
}

// example 4 : lifetime annotations cannot be ignored when implement methods
struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

impl PhoneModel {
    fn make_not_on_sale(&self) {
        self.on_sale.set(false);
    }
}

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}
