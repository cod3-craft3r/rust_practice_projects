mod pizza_order {
  pub struct Pizza {
    pub dough: String,
    pub sauce: String,
    pub cheese: String,
    pub topping: String
  }

  impl Pizza {
    pub fn lunch(topping: &str) -> Pizza {
      Pizza {
        dough: String::from("regular dough"),
        sauce: String::from("tomato"),
        cheese: String::from("mozzarella"),
        topping: String::from(topping)
      }
    }
  }

  pub mod help_customer {
    fn seat_at_table() {
      println!("customer seated at table");
    }

    pub fn take_order() {
      seat_at_table();
      let cust_pizza: super::Pizza = super::Pizza::lunch("veggies");
      serve_customer(cust_pizza);
    }

    fn serve_customer(cust_pizza: super::Pizza) {
      println!("the customer is served a regular pizza with {}", cust_pizza.topping);
    }
  }
}

pub fn order_food() {
  crate::restaurant::pizza_order::help_customer::take_order();
  // now we can reference this module in other files & just call the `order_food` function & this function will take care of the rest of it.
}