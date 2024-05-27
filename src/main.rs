fn main() {
    let apples = 10; // apples is 10
    println!("apples == {}", apples);
    {
        let apples = 20; // shadow apples in this scope
        println!("apples == {}", apples);
    }
    // That block's scope is done
    // Now our original apples is back in scope
    // What do you think this will output?
    println!("apples == {}", apples);
}
