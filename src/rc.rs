use std::{ops::Deref, rc::Rc};

pub fn play() {
    let s = WantedStrcut {
        value: String::from("some kinf of value"),
    };

    let rc = Rc::from(s);

    take_struct_ownership1(Rc::clone(&rc));
    take_struct_ownership2(Rc::clone(&rc));

    print!("{:#?}", rc.value);
}

#[derive(Debug)]
struct WantedStrcut {
    value: String,
}

impl Deref for WantedStrcut {
    type Target = WantedStrcut;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

fn take_struct_ownership1(wanted_struct: Rc<WantedStrcut>) -> () {
    print!("wanted_struct: #{wanted_struct:#?}")
}
fn take_struct_ownership2(wanted_struct: Rc<WantedStrcut>) -> () {
    print!("wanted_struct2: #{wanted_struct:#?}")
}
