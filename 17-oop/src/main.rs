#![allow(unused)]

use oop::Button;
use oop::Draw;
use oop::Screen;
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    println!("Hello, world!");
    let avgcollection = oop::AverageCollection {
        list: vec![],
        average: 1.0,
    };

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
