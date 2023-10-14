use std::ops::Add;
// Generics

// We cannot directly do x + y if we don't Add trait, 
// as that is not valid for every data type
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
   println!("5 + 4 = {}", get_sum_gen(5, 4));
   println!("2.2 + 3.7 = {}", get_sum_gen(2.2, 3.7));
}
