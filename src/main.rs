use rand::Rng;

// Random
fn main() {
    let random_num = rand::thread_rng().gen_range(1..101); // Generates a random no. between 1 and 100
    println!("Random number: {}", random_num);
}
