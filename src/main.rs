// Math
fn main() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111); // 1.2222223 (6 digits of precision)

    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111); // 1.2222222222222219 (14 digits of precision)

    let num_3: u32 = 5;
    let num_4: u32 = 4;

    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
}
