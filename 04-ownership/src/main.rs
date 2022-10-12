#![allow(unused)]
fn main() {
    //什么是所有权
    {
        fn takes_ownership(some_string: String) {
            // some_string 进入作用域
            println!("{}", some_string);
        } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

        fn makes_copy(some_integer: i32) {
            // some_integer 进入作用域
            println!("{}", some_integer);
        }

        fn gives_ownership() -> String {
            let some_string = String::from("yours");
            some_string
        }

        fn takes_and_gives_back(a_string: String) -> String {
            // a_string 进入作用域

            a_string // 返回 a_string 并移出给调用的函数
        }

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();
            return (s, length);
        }
        let s = "hello";
        {
            let s = "world";
        }
        let s = String::from("hello");

        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
        let x = 5;
        let y = x;
        println!("x is {}, y is {}", x, y);

        let s1 = String::from("hello");
        let s2 = s1; // s1 move to s2
        println!("s1 = {}", s2);

        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);

        let s = String::from("hello");
        takes_ownership(s);

        let x = 5;
        makes_copy(x);

        let s1 = gives_ownership(); // gives_ownership 将返回值
                                    // 移给 s1

        let s2 = String::from("hello"); // s2 进入作用域

        let s3 = takes_and_gives_back(s2); // s2 被移动到
                                           // takes_and_gives_back 中,
                                           // 它也将返回值移给 s3

        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
    // 引用和借用
    {
        fn calculate_length(s: &String) -> usize {
            s.len()
        }
        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }

        fn no_dangle() -> String {
            let s = String::from("hello");
            s
        }

        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        let mut s = String::from("hello");
        change(&mut s);

        let mut s = String::from("hello");
        {
            let r1 = &mut s;
        } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

        let r2 = &mut s;

        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{}, {}", r1, r2);

        let r3 = &mut s; // 没问题
        println!("{}", r3);
    }
    //切片 Slice 类型
    {
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }

        fn second_words(s: &String) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            return &s[..];
        }

        let mut s = String::from("hello world");
        let word = first_word(&s);
        println!("s length {} ", word);
        s.clear();
        println!("s {}", s);
        println!("s length {} ", word);

        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];

        let mut s = String::from("hello world");
        let word= second_words(&s);
        // s.clear(); //error
        println!("the first word is: {}", word);
        s.clear();
    }
}
