fn main() {
    let numbers: [i32; 5] = [2, 3, 8, 1, 9];
    let mut i = 0;

    while i < numbers.len() {
        println!("numbers[{}] == {}", i, numbers[i]);
        i += 1;
    }
}