// Structs
fn main() {
   struct Customer {
        name: String,
        address: String,
        balance: f32
   }

   let mut bob = Customer {
        name: String::from("Bob Marley"),
        address: String::from("555 Main St."),
        balance: 234.50
   };

   bob.balance = 123.20;

   println!("{}, {} - {}", bob.name, bob.address, bob.balance);

   // structs with generics
   struct Rectangle<T, U> {
        length: T,
        width: U
   }

   let rec = Rectangle {
        length: 1,
        width: 2.4
   };

   println!("{} {}", rec.length, rec.width);
}
