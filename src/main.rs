use std::cmp::Ordering;

// Conditionals
fn main() {
    let age = 8;

    if age >= 1 && age <= 18 {
        println!("Important Birthday");
    } else if age == 21 || age == 50 {
        println!("Little bit important birthday");
    } else if age >= 65 {
        println!("Least bit important birthday");
    } else {
        println!("Not an important birthday");
    }

    // Ternary operator in Rust
    let can_vote = if age >= 18 {
        true // No need of semi-colon
    } else {
        false
    };

    // Single-line ternary operator
    let can_study = if age <= 18 { true } else { false };

    println!("{} {}", can_vote, can_study);

    let age2 = 22;

    // Special kind of conditionals. More powerful than Switch cases
    // You can match ranges too. Will throw compile error if all possible
    // values are not handled
    match age2 {
        1..=18 => println!("Important Birthday"), // Matches range from 1 to 18 inclusive (If it was 1..18, the range would had been from 1 to 17)
        21 | 50 => println!("Little bit important birthday"), // Matches 21 or 50
        65..=i32::MAX => println!("Least bit important birthday"), // Matches 65 to Interger Max
        _ => println!("Not an important birthday") // Default value
    }

    let voting_age = 18;

    match age2.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Equal | Ordering::Greater => println!("Can vote")
    }
}
