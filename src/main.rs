// Functions

fn say_hello() {
    println!("Hello World");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum2(x: i32, y: i32) -> i32 {
    x + y // Equivalent to return x + y; Using semi-colon here is invalid
}

fn get_sum3(x: i32, y: i32) -> i32 {
    return x + y;
}

// To return multiple values, return a tuple (The parentheses are necessary)
fn get_square_and_cube(x: i32) -> (i32, i32) {
    return (x * x, x * x * x);
}

fn sum_list(list: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for num in list  {
        sum += num;
    }

    return sum;
}

fn main() {
   say_hello();
   get_sum(2, 3);
   println!("{} + {} = {}", 3, 4, get_sum2(3, 4));
   println!("{} + {} = {}", 6, 9, get_sum3(6, 9));

   let (square, cube) = get_square_and_cube(7);

   println!("The square and cube of {} is: {} and {}", 7, square, cube);

   let num_list = vec![1, 2, 3, 4, 5];
   println!("Sum of list = {}", sum_list(&num_list));
}
