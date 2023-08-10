use crate::MutableStuff;

pub async fn play(stuff: &mut MutableStuff) {
    stuff.mutation();
    println!("lalala")
}
