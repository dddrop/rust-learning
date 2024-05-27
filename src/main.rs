struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn price(&self) -> i32 {
        self.apples * 8 + self.bananas * 12
    }

    fn new() -> Self {
        Self {
            apples: 10,
            bananas: 5,
        }
    }
}

fn main() {
    let fruit = Fruit::new();
    let price = fruit.price();
    println!("Price is {}", price);
}
