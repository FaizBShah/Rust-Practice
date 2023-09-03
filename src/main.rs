// Casting
fn main() {
    let num1: u8 = 5;
    let num2: u8 = 4;

    let num3: u32 = num1 as u32; // Casting u8 to u32 using 'as' keyword
    let num4: u32 = (num1 as u32) + (num2 as u32);

    println!("{} {}", num3, num4);
}
