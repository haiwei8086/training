use std::{mem};


fn main() {
    println!("Size test");

    println!("Bool size: {}", mem::size_of::<bool>());

    println!("i32:123 size: {}", mem::size_of::<i32>());


    println!("u16:10 size: {}", mem::size_of::<[u16; 10]>());

    let str: &str = "master";
    println!("&str:master size: {}", mem::size_of_val(&str));

}