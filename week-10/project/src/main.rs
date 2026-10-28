struct Electronics {
    brand: String,
    quantity: u32,
    price_each: u32
}

impl Electronics {
    fn cost(&self) -> u32  {
        3*self.price_each
    }
    
}

fn main() {
    let b1 = Electronics{
        brand: String::from("HP"),
        quantity: 6,
        price_each: 650_000
    };
    let b2 = Electronics{
        brand: String::from("IBM"),
        quantity: 10,
        price_each: 755_000
    };
    let b3 = Electronics{
        brand: String::from("Toshiba"),
        quantity: 10,
        price_each: 550_000
    };
    let b4 = Electronics{
        brand: String::from("Dell"),
        quantity: 4,
        price_each: 850_000
    };
    let total_cost = b1.cost()+ b2.cost()+ b3.cost()+ b4.cost();
    println!("Total cost is {}",total_cost);
}
