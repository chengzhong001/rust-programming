#![allow(unused)]

fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    {
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));
        f();
        fn takes_long_type(f: Thunk) {
            // --snip--
        }
    }
}
