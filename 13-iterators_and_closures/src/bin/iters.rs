#![allow(unused)]

fn main() {
    let mut v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
      
        println!("Got: {}", val);
    }

    let v1_iter_mut = v1.iter_mut();

    for val in v1_iter_mut {
        *val += 1;
    }

    let v1_into_iter = v1.into_iter();
    for val in v1_into_iter{
        println!("Got: {}", val);
    }

    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);
  
}
