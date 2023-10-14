// Ownership

// RULES:
// 1. Each value has a variable that is called its owner
// 2. There is only one owner at a time
// 3. When the owner goes out of scope, the value disappears

fn print_str(x: String) {
    println!("A string: {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string: {}", x);
    return x;
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("{}", name);
}

fn main() {
   let str1 = String::from("World");
   let str2 = str1; // Borrowing value and taking ownership from str1

   // println!("Hello {}", str1); This will throw error as value stored in str1 has been borrowed by str2, and str1 is no longer the owner of that string
   println!("Hello {}", str2); // Will work fine

   let str3 = String::from("Kolkata");
   let str4 = str3.clone(); // Returning a clone of str1

   // Both will work
   println!("Good morning, {}!!", str3);
   println!("Good morning, {}!!", str4);

   // This borrowing doesn't affect primitive types, or types which are stored in stack
   let num1 = 2;
   let num2 = num1;

   println!("{} {}", num1, num2); // Both working

   let str5 = String::from("Faiz");

   // print_str(str5); This works fine
   let mut str6 = print_return_str(str5); // This will fail if above line is uncommented. This is because str5 ownership is passed to the function. To fix this, instead pass a refernce to the function using &str5
   println!("{}", str3);

   change_string(&mut str6); // Passing mutable reference
}
