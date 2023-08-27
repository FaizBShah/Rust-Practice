// Arrays and Loops
fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("{} {} {} {} {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
    println!("Length: {}", arr.len());
    
    // For loop through array
    println!("Printing all elements of the array using for loop");
    for num in arr {
        println!("{}", num);
    }

    // Declaring loop variable
    let mut i = 0;

    // While loop through array
    println!("Printing all elements of the array using while loop");
    while i < arr.len() {
        println!("{}", arr[i]);
        i += 1;
    }

    i = 0;

    // Printing only the even no.s in the array
    println!("Printing the even no.s of the array");
    loop {
        if i == arr.len() {
            break;
        }

        if arr[i] % 2 == 0 {
            println!("{}", arr[i]);
        }

        i += 1;
    }
}
