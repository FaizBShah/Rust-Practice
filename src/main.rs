use rand::Rng;
use std::io::{self, Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Types of variables
fn main() {
    const ONE_MILLION: i32 = 1_000_000; // Integer constant
    const PI: f32 = 3.141592; // Float constant

    let age = "47"; // String variable
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a no."); // Shadowing
    age = age + 1;

    print!("I'm {} and I want ${}", age, ONE_MILLION);
}
