// HashMaps
use std::collections::HashMap;

fn main() {
   let mut heroes = HashMap::new();

   // Inserting values in HashMap
   heroes.insert("Superman", "Clark Kent");
   heroes.insert("Batman", "Bruce Wayne");
   heroes.insert("Ironman", "Tony Stark");

   for (k, v) in heroes.iter() {
       println!("{} - {}", k, v);
   }

   println!("The size of hash map is = {}", heroes.len()); // 3
   
   println!("{}", heroes.contains_key("Superman")); // true
   println!("{}", heroes.contains_key("Spiderman")); // false

   heroes.remove("Superman");

   println!("The size of hash map is = {}", heroes.len()); // 2
   println!("{}", heroes.contains_key("Superman")); // false

   if heroes.contains_key("Batman") {
        let batman_real_name = heroes.get("Batman");
        match batman_real_name {
            Some(x) => println!("The real name of Batman is {}", x),
            None => println!("Name not present")
        }
   }
}
