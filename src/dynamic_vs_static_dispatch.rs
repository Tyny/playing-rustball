use std::ops::Deref;

use crate::build_message;

pub fn play() {
    let greeter1 = DefaultGreeter {};
    let greeter2 = LunfardoGreeter {};

    let greeters: Vec<Box<dyn Greeter>> = vec![Box::new(greeter1), Box::new(greeter2)];

    let greeters2: Vec<EnumGreeter> = vec![
        EnumGreeter::LunfardoGreeter(),
        EnumGreeter::DefaultGreeter(),
    ];

    println!("{}", apply_greeters(greeters));
    println!("{}", apply_greeters2(greeters2));
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

fn apply_greeters<T: Deref<Target = dyn Greeter>>(greeters: Vec<T>) -> String {
    build_message!(greeters) // maybe overkill here using this macro but it was fun
}

fn apply_greeters2(greeters: Vec<EnumGreeter>) -> String {
    build_message!(greeters) // maybe overkill here using this macro but it was fun
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
