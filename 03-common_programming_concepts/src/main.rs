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
}
