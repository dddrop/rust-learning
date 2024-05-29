#[derive(Clone)]
struct Fruit<T> {
    apples: T,
    bananas: T,
}

fn print_fruit(fruit: Fruit<i32>) {
    println!("Apples: {}, bananas: {}", fruit.apples, fruit.bananas);
}

fn main() {
    let mut fruit = Fruit {
        apples: 5,
        bananas: 10,
    };
    print_fruit(fruit.clone());
    fruit.apples *= 2;
    fruit.bananas *= 3;
    print_fruit(fruit);
}
