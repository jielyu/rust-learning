fn main() {
    test_raw_pointer();
    test_unsafe_function();
    test_mut_static();
}

fn test_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 += 1;
        println!("r2 is: {}", *r2);
    }
}

use std::slice;
fn split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = s.len();
    let ptr = s.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}
fn test_unsafe_function() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("a={:?}, b={:?}", a, b);
}

static mut COUNTER: u32 = 0;
fn test_mut_static() {
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {}
unsafe impl Foo for i32 {}
