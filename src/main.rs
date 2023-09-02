// Strings
fn main() {
    // There are two types of strings in Rust.
    // One is the String type, which is similar to StringBuilder in Java.
    // Its internally represented as a vector of bytes, and provides functions to mutate strings
    // The second is the &str type, which represents an immutable, borrowed view into a String.
    // It has fixed size of characters, and you cannot modify the underlying data using &str
    let mut str: String = String::new(); // Need to add mut keyword if you want to make String mutable

    str.push('A'); // To append a character
    str.push_str("pples are awesome"); // To append a string

    // Printing all the words in a string
    for word in str.split_whitespace() {
        println!("{}", word);
    }

    let str2 = str.replace("Apples", "Mangoes");
    println!("{}", str2);

    let str3 = String::from("unprosperousness"); // Creating a string with an initial value
    let mut v: Vec<char> = str3.chars().collect();

    v.sort(); // Sorts the vector
    v.dedup(); // Removes consecutive duplicates. Hence, if vector is sorted, removes all duplicates

    for c in v {
        println!("{}", c);
    }

    let str4: &str = "Random string";
    let mut str5: String = str4.to_string(); // To convert &str to String
    println!("{} {}", str4, str5);

    let byte_arr: &[u8] = str5.as_bytes();
    let str6: &str = &str5[0..6]; // 0 to 5

    for byte_char in byte_arr {
        println!("{}", byte_char);
    }

    println!("{}", str6);
    println!("{} {}", str5.len(), str6.len());

    str5.clear(); // Deletes the entire string

    let str7 = String::from("Just some");
    let str8 = String::from(" words");
    // str7 will be unusable after this because the value now belongs to str9.
    // But same is not for the case for str8, as we are using its reference instead
    // of using it directly
    let str9: String = str7 + &str8;
    println!("{}", str9);

    for c in str9.chars() {
        println!("{}", c);
    }
}
