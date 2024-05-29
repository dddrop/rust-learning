trait Double {
    fn double(self) -> Self;
}

impl Double for i32 {
    fn double(self) -> i32 {
        self * 2
    }
}

fn info<T>(x: T)
where
    T: Double + std::fmt::Display + Copy,
{
    println!("Original number: {}", x);
    println!("Doubled number: {}", x.double());
    println!("Quadrupled number: {}", x.double().double());
}

fn main() {
    info(5);
}
