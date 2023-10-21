// Iterators
fn main() {
    let mut arr = [1, 2, 3, 4];

    for val in arr.iter() {
        println!("{}", val); // Can read the values
    }

    for val in arr.iter_mut() {
        *val *= 2; // Can update the value
    }

    println!("{:?}", arr);

    let mut iter = arr.iter();

    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // Some(4)
    println!("{:?}", iter.next()); // Some(6)
    println!("{:?}", iter.next()); // Some(8)
    println!("{:?}", iter.next()); // None
}
