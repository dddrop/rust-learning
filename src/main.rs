#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30,
    };

    let also_alice = alice.clone();
    assert_eq!(alice, also_alice);
    assert!(alice >= also_alice);
    assert!(alice <= also_alice);

    let bob = Person {
        name: "Bob".to_owned(),
        age: 25,
    };

    assert_ne!(alice, bob);

    // What do you think is bigger, Alice or Bob?
    println!("{:?} > {:?} == {}", alice, bob, alice > bob);
}
