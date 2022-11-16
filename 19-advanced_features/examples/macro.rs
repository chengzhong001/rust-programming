#![allow(unused)]

#[macro_export]
macro_rules! vec1 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


fn main(){
    let v = vec![1,2,3];
    let v = vec1!(1,2,3);
    
    println!("{:?}", v);
}