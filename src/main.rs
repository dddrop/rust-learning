struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit(mut fruit: Fruit) -> Fruit {
    fruit.apples *= 2;
    fruit.bananas *= 3;
    fruit
}

fn print_fruit(fruit: Fruit) -> Fruit {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
    fruit
}

fn main() {
    let fruit = Fruit {
        apples: 10,
        bananas: 5,
    };

    let fruit = print_fruit(fruit);
    let fruit = increase_fruit(fruit);
    print_fruit(fruit);
}
