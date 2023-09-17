// Vectors
fn main() {
   let _vec1: Vec<i32> = Vec::new(); // Immutable vector
   let mut vec2: Vec<i32> = vec![1, 2, 3, 4];
   vec2.push(5);
   println!("{}", vec2[0]);

   let _second: &i32 = &vec2[1];

   match vec2.get(1) {
       Some(value) => println!("{}", value),
       None => println!("No second value")
   }
   
   for i in &mut vec2 {
       *i *= 2;
   }

   for i in &mut vec2 {
        println!("{}", i);
   }

   println!("Length: {}", vec2.len());
   println!("Pop: {:?}", vec2.pop());
}
