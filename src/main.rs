// Enums
// You can define enums outside of a function...
enum Suits {
    Heart,
    Diamond,
    Spade,
    Club
}

fn main() {
    //...or inside!!
    enum Day {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday
    }

    // To declare a method for an enum
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sunday | Day::Saturday => true,
                _ => false
            }
        }
    }

    println!("{}", Day::Sunday.is_weekend());
    println!("{}", Day::Monday.is_weekend());
    println!("{}", Day::Tuesday.is_weekend());
    println!("{}", Day::Wednesday.is_weekend());
    println!("{}", Day::Thursday.is_weekend());
    println!("{}", Day::Friday.is_weekend());
    println!("{}", Day::Saturday.is_weekend());

    let today: Day = Day::Monday;
    
    println!("{}", today.is_weekend());
}
