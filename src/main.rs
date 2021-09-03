use std::slice;

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
}
