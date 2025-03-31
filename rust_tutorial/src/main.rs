#![allow(unused)]

use core::panic;
use std::io;
use std::fs::File;
use std::io::{ Write, BufRead, BufReader, ErrorKind };
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    println!("What's your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    
    // this is how you read an input in Rust. By defalt, Rust has an inbuilt error-handling & thus it's stdin returns a Result which contains either Ok or an Error value depending upon whatever the current use case is.
    // io::stdin().read_line(&mut name)
    //     .expect("Didn't Receive Input");

    // println!("Hello, {}! {}", name.trim_end(), greeting);

    // BEST PRACTICES: this is how you should declare constants in Rust. Fun fact, Rust allows you to initialize big values using a separator like `_` so that it increases code readability.
    const ONE_MIL: u32 = 1_000_000_000;
    const PI: f32 = 3.141592;

    // Shadowing in Rust is where you declare variables with the same name but different data types. 
    let age: &str = "21";
    // let mut age: u32 = 42;
    let mut age: u32 = age.trim_end().parse()
        .expect("Age wasn't assigned a number");

    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    // let's generate some random numbers
    let random_num = rand::thread_rng().gen_range(1..11);
    println!("random number generated between 1 & 10 - {}", random_num);


    // match - more like enhanced switch-case
    let age2: u32 = 18;
    match age2 {
        1..=18 => println!("you have an important birthday!"),
        21 | 50 => println!("you have an important birthday!"),
        65..=u32::MAX => println!("you have an important birthday!"),
        _ => println!("not an important birthday"),
    };

    let my_age = 19;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("you just got the right to vote!")
    };

    let arr = [1,2,3,4,5];
    println!("length - {}", arr.len());

    for val in arr.iter() {
        println!("arr - {}", val);
    }

    // now, let's learn tuples - remember that DS that can store multiple data types in it?
    let my_tuple: (u8, String, f64) = (42, "Archit".to_string(), 50.0000);
    println!("name-{}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;
    println!("num-{}", v1);

    // moving on to strings, there are two types of it in Rust: String (which is a vector of bytes that is mutable) & &str (which is read-only)
    let mut st1 = String::new();
    st1.push('A');
    st1.push('B');
    st1.push_str(" Bosworth");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace(" Bosworth", "CD");
    println!("{}", st2);

    // more string operations...
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1 {
        print!("{}", char);
    }

    let st4 = "Random string";
    let mut st5 = st4.to_string();
    println!("{}",st5);

    let byte_arr = st5.as_bytes();  //converting string into a byte array just like a mutable string is really stored in the memory
    for byte in byte_arr {
        print!("{}",byte);
    }

    let st6 = &st5[0..=5];  //slicing a string
    println!("length of the sliced string-{}", st6.len());

    // let's now learn casting! -> switching from one data type to another
    // we already know that Rust allows to typecast using the keyword `as`, we'll learn enums now
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today = Day::Monday;
    println!("is today a weekend? {}", today.is_weekend());

    // vectors
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4,5];
    vec2.push(6);
    let second = &vec2[1];

    match vec2.get(1) {
        Some(second) => println!("2nd-{}",second),
        None => println!("no second value")
    }

    // referencing to a variable or DS in Rust is done in two ways: mutable & immutable way

    // this is a mutable reference (&mut) to the vector where we CAN modify the elements of the vector directly
    for i in &mut vec2 {
        *i *= 2;
    }

    // this is an immutable reference (&) to the elements of vector where we're only supposed to read the values stored inside it
    for i in &vec2 {
        println!("{}", i);
    }

    println!("popped-{:?}", vec2.pop());

    // OWNERSHIP in Rust - it's a big feature of the language; only one ownwer at a time can exist in the scope!
    let str1 = String::from("World");
    // let str2 = str1;
    // println!("Hello, {}", str1);     // so we now can't access the value at str1 because it doesn't exist now...it's value was borrowed to another variable str2 in this case & since only one owner exists at a time, str1 is removed from the scope.
    // to avoid that fuckery, use...
    let str2 = str1.clone();

    // passing variables as reference is somewhat like C++ except a few syntactical changes here & there...
    let mut str3 = String::from("Hello");
    change_str(&mut str3);

    println!("{}", str3);

    // let's now use hash maps...
    let mut heroes: HashMap<&str, &str> = HashMap::new();

    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    print!("the map has - {} items\n", heroes.len());

    for(k, v) in heroes.iter() {
        println!("{} = {}", k,v);
    }
    if heroes.contains_key("Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batmna isn't a hero")
        }
    }
    
    // let's now play with custom data types: structs
    struct Customer {
        name: String,
        address: String,
        balance: f32
    }

    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main St."),
        balance: 234.55
    };

// we're only able to change the addresss since we've declared bob to be a mutable variable
    bob.address = String::from("555 Jane St.");

    // structs with generics...
    // struct Rectangle<T, U> {
    //     length: T,
    //     bredth: U
    // }

    // let rec = Rectangle{length: 4, bredth: 10.5};
    // the compiler is so smort that it automatically recognizes the change in the two data types.

    // Rust also supports OOP...
    trait Shape {
        fn new(length: f32, bredth: f32) -> Self;   //this is a constructor
        fn area(&self) -> f32;
    }

    struct Rectangle {length: f32, bredth: f32};
    struct Square {length: f32};
    
    impl Shape for Rectangle {
        fn new(length: f32, bredth: f32) -> Rectangle {
            return Rectangle { length, bredth };
        }

        fn area(&self) -> f32 {
            return self.length * self.bredth;
        }
    }

    impl Shape for Square {
        // this is kind of a useless eg since here we require length as well as bredth but square doesn't have one - it's just a demo
        fn new(length: f32, bredth: f32) -> Self {
            return Square { length };
        }

        fn area(&self) -> f32 {
            return (self.length).powf(2.0);
        }
    }

    let rec: Rectangle = Shape::new(10.0, 12.0);
    let sq: Square = Shape::new(10.0, 10.0);

    println!("rectangle's area - {}", rec.area());
    println!("square's area - {}", sq.area());

    // Crates - modules that produce a library or executable
    // Modules - organize & handle privacy
    // Packages - build, test, & share crates
    // Paths - a way of naming an item such as struct or a function
    order_food();

    // now, error handling is not something unique to Rust, it doesn't really explicitly handle the errors like some other languages do (handling exceptions of different categories). In fact, Rust handles recoverable errors with the `Result<>` as seen before & for the unrecoverable errors, it uses the panic macro!
    // we'll experience error handling along with files in this example...

    let path = "lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("problem creating file {:?}", error)
        }
    };

    write!(output, "just some\nrandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffer = BufReader::new(input);

    for line in buffer.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(file) => file,
                Err(e) => panic!("can't create file {:?}", e)
            },
            _other_error => panic!("problem opening file {:?}", error)
        }
    };

    // CLOSURES- are basically functions without names (anonymous fn ig) these are stored in a variable, not so anonymous huh?
    let can_vote = |age: i32| -> bool {age >= 18};
    println!("can vote - {}", can_vote(8));

    // one weird thing that closures can do is that access variables outside their own scope...
    let mut sample1 = 10;
    println!("sample1 - {}", sample1);
    let mut change_var = || sample1 += 1;
    change_var();
    println!("sample1 - {}", sample1);
    sample1 = 10;
    println!("sample1 - {}", sample1);

    // closures with generics...
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }

    let sum = |a, b| a+b;
    let prod = |a, b| a*b;

    println!("sum is - {}", use_func(3, 5, sum));
    println!("product is - {}", use_func(3, 5, prod));
}

fn change_str(x: &mut String) {
    x.push_str(" World");
}