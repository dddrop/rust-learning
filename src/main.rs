#[derive(Debug)]
struct Fruit {
    apples: i32,
    bananas: i32,
}

fn main() {
    let fruit = Fruit {
        apples: 5,
        bananas: 10,
    };
    println!("{:?}", fruit);
}
