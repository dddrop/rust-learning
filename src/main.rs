fn main() {
    let speed = Mph { value: 90 };
    let distance = speed.in_three_hours();
    println!("At {:?}, you will travel {:?} in 3 hours", speed, distance);
}

#[derive(Debug, Clone, Copy)]
struct Kmh {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
struct Km {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
struct Mph {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
struct Miles {
    value: i32,
}

trait InThreeHours {
    type Distance;
    fn in_three_hours(&self) -> Self::Distance;
}

impl InThreeHours for Kmh {
    type Distance = Km;
    fn in_three_hours(&self) -> Km {
        Km {
            value: self.value * 3,
        }
    }
}

impl InThreeHours for Mph {
    type Distance = Miles;
    fn in_three_hours(&self) -> Miles {
        Miles {
            value: self.value * 3,
        }
    }
}
