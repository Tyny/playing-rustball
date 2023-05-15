use std::ops::Deref;

fn main() {
    let greeter1 = DefaultGreeter {};
    let greeter2 = LunfardoGreeter {};

    let greeters: Vec<Box<dyn Greeter>> = vec![Box::new(greeter1), Box::new(greeter2)];

    let greeters2: Vec<EnumGreeter> = vec![
        EnumGreeter::LunfardoGreeter(),
        EnumGreeter::DefaultGreeter(),
    ];

    println!("{}", say_hello(greeters));
    println!("{}", say_hello2(greeters2));
}

enum EnumGreeter {
    LunfardoGreeter(),
    DefaultGreeter(),
}

impl EnumGreeter {
    fn hi(&self) -> String {
        match self {
            EnumGreeter::DefaultGreeter() => "Enum default greeter saying hi!".to_string(),
            EnumGreeter::LunfardoGreeter() => {
                "Que hace chabonete dice el Enum Lunfardo greeter".to_string()
            }
        }
    }
}

pub trait Greeter {
    fn hi(&self) -> String;
}

fn say_hello<T: Deref<Target = dyn Greeter>>(greeters: Vec<T>) -> String {
    let iter = greeters.into_iter();

    let mut great_greet = "".to_string();
    for greet in iter {
        let g = greet.hi();
        great_greet.push_str("\n");
        great_greet.push_str(&g);
    }

    great_greet.to_string()
}

fn say_hello2(greeters: Vec<EnumGreeter>) -> String {
    let iter = greeters.into_iter();

    let mut great_greet = "".to_string();
    for greet in iter {
        let g = greet.hi();
        great_greet.push_str("\n");
        great_greet.push_str(&g);
    }

    great_greet.to_string()
}

struct DefaultGreeter;

impl Greeter for DefaultGreeter {
    fn hi(&self) -> String {
        "I'm the default greeter!".to_string()
    }
}

struct LunfardoGreeter;

impl Greeter for LunfardoGreeter {
    fn hi(&self) -> String {
        "Que hace' chabooon".to_string()
    }
}
