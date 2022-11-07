#![allow(unused)]

// use std::fmt::Display;
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str){
    println!("Hello, {}", name);
}


fn main() {
    let x = 5;
    let y = &x;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x = {:p}", &x);
    println!("y = {:p}", y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x = {}", x);
    println!("x = {}", *y);
    println!("y = {}", *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}
