struct Person {
    name: String,
    age: u32,
    job: Job,
}

#[derive(PartialEq, Eq)]
enum Job {
    Teacher,
    Scientist,
    Chef,
}
impl Person {
    fn same_job(&self, other: &Person) -> bool {
        self.job == other.job
    }
}

impl Person {
    fn greeting(&self) -> String {
        match self.job {
            Job::Teacher => {
                format!("Hello, you're a teacher named {}", self.name)
            }
            Job::Scientist => {
                format!("Hello, you're a scientist named {}", self.name)
            }
            Job::Chef => {
                format!("Hello, you're a chef named {}", self.name)
            }
        }
    }
}

fn main() {
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30,
        job: Job::Chef,
    };
    println!("{}", alice.greeting());
}
