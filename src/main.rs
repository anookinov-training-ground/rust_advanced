#![allow(unused)]
use std::slice;
use std::ops::Add;
use std::fmt;
use std::io::Error;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // let len = slice.len();

    // assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..])

    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);;
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {} // can't implement OutlinePrint on a type that doesn't implement Display

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

type Result<T> = std::result::Result<T, std::io::Error>;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn bar() -> ! {
    // --snip--
    panic!();
}

enum CustomOption<T> {
    Some(T),
    None,
}

use crate::CustomOption::*;

impl<T> CustomOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    // println!("r1 is: {}", *r1); // can't dereference raw pointers outside of unsafe block

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // dangerous(); // can't call unsafe function outside of unsafe block

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);

    let address = 0x01234usize;
    let r = address as *mut i32;

    // let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) }; // would likely crash

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    trait Add<Rhs=Self> {
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // Human::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()); // error as Rust can't figure out which implementation for Animal::baby_name we want
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // fully qualified syntax i.e. <Type as Trait>::function(receiver_if_method, next_arg, ...);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| ())
    }

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_type_alias(f: Thunk) {
        // --snip--
    }

    fn returns_type_alias() -> Thunk {
        // --snip--
        Box::new(|| ())
    }

    // let guess = "3";
    // let guess = match guess.trim().parse() {
    //     Ok(_) => 5,
    //     Err(_) => "hello", // doesn't work as match arms must all return the same type
    //     // Err(_) => continue, // OK as continue return ! (never type which never returns)
    // };

    print!("forever ");

    // loop has ! type but wouldn't be true if break is included
    // loop {
    //     print!("and ever ");
    // }

    // can't compile as rust needs to know size at compile time and str is a dynamically sized types
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // Trait is a dynamically sized type i.e. &dyn Trait, Box<dyn Trait>, Rc<dyn Trait>

    fn generic<T>(t: T) { // fn generic<T: Sized>(t: T) {}
        // -- snip--
    }

    // use ? to relax not to always have a known size at compile time
    fn generic_relax<T: ?Sized>(t: &T) {
        // --snip--
    }

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // function pointers implement all three of the closure traits (Fn, FnMut and FnOnce) so can be passed as an argument for a function that expects a closure
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings_fn: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    // initializer syntax ()
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // can't return closure as it's a trait
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    // use a trait object
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    let v: Vec<u32> = vec![1, 2, 3];
}

#[macro_export]
macro_rules! vec_macro {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec
// }