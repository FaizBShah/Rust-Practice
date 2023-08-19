// Data Types
fn main() {
    // INTEGER AND FLOAT DATA TYPES
    // Unsigned integer: u8, u16, u32, u64, u128, usize(Depends upon the size of the computer, e.g.: if 64-bit, then usize = 64-bit)
    // Signed integer: i8, i16, i32, i64, i128, isize(Depends upon the size of the computer, e.g.: if 64-bit, then isize = 64-bit)
    println!("Max i8: {}", i8::MIN);
    println!("Min i16: {}", i16::MIN);
    println!("Min i32: {}", i32::MIN);
    println!("Min i64: {}", i64::MIN);
    println!("Min i128: {}", i128::MIN);
    println!("Min isize: {}", isize::MIN);
    println!("Min u8: {}", u8::MIN);
    println!("Min u16: {}", u16::MIN);
    println!("Min u32: {}", u32::MIN);
    println!("Min u64: {}", u64::MIN);
    println!("Min u128: {}", u128::MIN);
    println!("Min usize: {}", usize::MIN);
    println!("Min f32: {}", f32::MIN);
    println!("Min f64: {}", f64::MIN);
    println!("Max i8: {}", i8::MAX);
    println!("Max i16: {}", i16::MAX);
    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);
    println!("Max i128: {}", i128::MAX);
    println!("Max isize: {}", isize::MAX);
    println!("Max u8: {}", u8::MAX);
    println!("Max u16: {}", u16::MAX);
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    // BOOLEAN DATA TYPES
    let _is_true = true; // Use underscore to ignore unused warning
    let is_false = false;

    // CHARACTER DATA TYPES
    let my_grade = 'A';
}
