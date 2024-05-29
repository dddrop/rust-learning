struct Person {
    name: String,
    age: u32,
    job: Job,
}

#[derive(PartialEq, Eq)]
enum Subject {
    Math,
}

#[derive(PartialEq, Eq)]
enum Job {
    Teacher(Subject),
    Scientist,
    Chef,
}

impl Job {
    fn is_teacher(&self) -> bool {
        match self {
            Job::Teacher(_) => true,
            Job::Scientist => false,
            Job::Chef => false,
        }
    }
}

impl Person {
    fn same_job(&self, other: &Person) -> bool {
        self.job == other.job
    }
}

impl Person {
    fn greeting(&self) -> String {
        match self.job {
            Job::Teacher(_) => format!("Hello, you're a teacher named {}", self.name),
            Job::Scientist => format!("Hello, you're a scientist named {}", self.name),
            Job::Chef => format!("Hello, you're a chef named {}", self.name),
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
