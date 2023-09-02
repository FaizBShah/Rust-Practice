// Tuples
fn main() {
    // Tuples are immutable, so it won't mutate even with the mut keyoword
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;

    println!("{}", v1); // 47
    println!("{}", v2); // "Derek"
    println!("{}", v3); // 50000
}
