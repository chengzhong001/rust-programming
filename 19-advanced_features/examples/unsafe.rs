#![allow(unused)]

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
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

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;

}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world";

static mut COUNTER:u32 = 0;

fn add_to_count(inc: u32){
    unsafe{
        COUNTER += inc;
    }
}
unsafe trait Foo {
    
}

unsafe impl Foo for i32 {
    
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {:?}", r1);
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        dangerous();
    }

    let mut array = [1, 2, 3, 4, 8, 9];
    let mid = &array.len() / 2;
    let (front, back) = split_at_mut(&mut array, mid);
    println!("front is {:?}", front);
    println!("back is {:?}", back);

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe{
        println!("COUNTER: {}", COUNTER);
    }
}
