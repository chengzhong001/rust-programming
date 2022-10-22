#![allow(unused)]
mod math;
use math::add;
use std::collections::HashMap;

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};
use std::collections::*;

use packages_crate_modules::eat_at_restaurant;
use packages_crate_modules::hosting;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    eat_at_restaurant();
    println!("3 + 4 = {}", add(3, 4));
}
