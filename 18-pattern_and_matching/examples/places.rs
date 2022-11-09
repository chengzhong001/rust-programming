#![allow(unused)]

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
fn main() {
    // if let 

    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for 
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate()  {
        println!("{} is at index {}", value, index);
    }

    // let
    let (x, y, z) = (1,2,3);

    let point = (3, 5);
    print_coordinates(&point);
}
