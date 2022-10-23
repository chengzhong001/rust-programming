#![allow(unused)]

use std::{ops::Add, fmt::format};

fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('a');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = s1.add(s2.as_str());
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}",s1, s2, s3);
    println!("s is {}", s);

    let len = String::from("Hola").len();
    println!("len is {}", len);

    let len = String::from("Здравствуйте").len();
    println!("len is {}", len);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    

}
