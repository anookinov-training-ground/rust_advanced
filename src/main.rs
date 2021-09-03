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
}
