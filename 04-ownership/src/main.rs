fn main() {
    let s = "hello";{
        let s = "world";
        println!("s value is {}", s);
    }
    println!("s value is {}", s);
}
