#![allow(unused)]
use std::mem;
struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}
fn main(){
    let c = CustomSmartPointer { data: String::from("my stuff") };
    mem::drop(c); // 提前清理
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}