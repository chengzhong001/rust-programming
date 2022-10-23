#![allow(unused)]
fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    {
        let v = vec![1, 2, 3, 4];
        println!("v[0] = {}", v[0]);
    }
    println!("v[0] = {}", v[0]);

    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    };
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i)
    }
    for (i, e) in v.iter().enumerate() {
        println!("index={}, element={}", i, e);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}
