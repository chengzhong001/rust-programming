fn main() {
    // 变量和不可变性
    {
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
        println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);

        let x = 5;
        let x = x + 1;
        {
            let x = x + 2;
            println!("The value of x in the inner scope is: {}", x);
        }
        println!("The value of x is: {}", x);
        let spaces = "   ";
        let spaces = spaces.len();
        println!("space length is {}", spaces);
    }
    // 数据类型
    {
        let guess: u32 = "42".parse().expect("Not a number"); // parse需要特别标注变量类型
        println!("guess is {}", guess);
        let x = 2.0;
        let y: f32 = 3.0;
        println!("x, y are {}, {}", x, y);
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0

        // remainder
        let remainder = 43 % 5;

        println!(
            "operation is {}, {}, {}, {}, {} ,{} ",
            sum, difference, product, quotient, floored, remainder
        );

        let t = true;
        let f: bool = false;
        println!("bool is {}, {}", t, f);
        // with explicit type annotation
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("char is {}, {} ,{} ", c, z, heart_eyed_cat);
        // 元组类型
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("tuple value is {}, {}, {}", x, y, z);

        // 数组类型
        let a = [1, 2, 3, 4, 5];
        println!("1st of a is {}", a[0]);

        let a = [3; 5]; // 5个3
        println!("1st of a is {}", a[0]);

        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        println!("2nd of a is {}", months[1]);

        let a = [1, 2, 3, 4, 5];
        // println!("The value of the element at index {} is: {}", 10, a[10]); //index out of bounds
        println!("The value of the element at index {} is: {}", 3, a[3]);
    }
    // 函数
    {
        fn another_function() {
            println!("Another function.");
        }
        another_function();

        fn another_function1(x: i32) {
            println!("The value of x is: {}", x);
        }
        another_function1(5);

        fn print_labeled_measurement(value: i32, unit_label: char) {
            println!("The measurement is: {}{}", value, unit_label);
        }

        print_labeled_measurement(5, 'x');

        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {}", y);

        fn five() -> i32 {
            return 5; // return 可以省略
        }
        println!("five function is {}", five());

        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        println!("plus_one function is {}", plus_one(5));
    }
    {
        // if else 语句
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
        let number = 3;
        if number != 0 {
            println!("number was something other than zero");
        }

        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        let condition = true;
        let number = if condition { 5 } else { 6 }; // condition 后面接的值必须是同类型

        println!("The value of number is: {}", number);

        //loop 循环
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;
            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up; // 注意：只有左边的单引号
                }
                remaining -= 1;
            }
            count += 1
        }
        println!("End count = {}", count);

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {}", result);

        // while循环
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }
        // for 循环
        let a = [10, 20, 30, 40, 50];

        for (index, element) in a.iter().enumerate() {
            println!("the index is {}, the value is: {}", index, element);
        }
    }
}
