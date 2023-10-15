mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("Regular dough"),
                cheese: String::from("Mozarella"),
                topping: String::from(topping)
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }

        fn serve_customer(cust_pizza: super::Pizza) {
            println!("The customer is served pizza with topping: {}", cust_pizza.topping);
        }

        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(cust_pizza);
        }
    }
}

pub fn order_food() {
    pizza_order::help_customer::take_order();
}