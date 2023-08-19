// #![allow(unused)]

use rand::Rng;
use std::io::{self, Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Taking input from standard input and printing it
fn main() {
    println!("What is your name?");

    let mut name= String::new();
    let greeting = "Nice to meet you";

    io::stdin().read_line(&mut name).expect("Didn't receive Input");
    
    print!("Hello {}! {}", name.trim_end(), greeting);
}
