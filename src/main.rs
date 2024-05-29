struct OneToTen {
    // This will be the next value when we call next
    next_val: u32,
}

impl OneToTen {
    // Start off at 1
    fn new() -> Self {
        OneToTen { next_val: 1 }
    }
}

impl Iterator for OneToTen {
    // Creating a stream of u32 values
    type Item = u32;

    // Could also be -> Option<u32>
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_val > 10 {
            // Already got to 10, stop
            None
        } else {
            // Capture the current value in a variable
            let result = Some(self.next_val);
            // Increase the next value by 1. This is where we
            // see mutation happening.
            self.next_val += 1;
            // Return the previously captured value.
            result
        }
    }
}

fn main() {
    let mut total = 0;

    // Works just like 1..11
    for i in OneToTen::new() {
        total += i;
    }

    assert_eq!(total, 55);
    println!("Success!");
}