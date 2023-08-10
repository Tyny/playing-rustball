use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, RwLock},
};

use actix::Actor;
use actors::{DispatchQueueWorker, FutureMessage};

mod actors;
mod dynamic_vs_static_dispatch;
mod macros;
mod mutex;
mod pointers;
mod rc;

pub struct MutableStuff {
    value: Option<String>,
}

impl MutableStuff {
    pub fn mutation(&mut self) {
        self.value = Some("a".to_string())
    }
}

#[actix_rt::main]
async fn main() {
    //rc::play();

    // shareable mutable containers std:cell
    // this is referred as interior mutability - a type that looks like it is inmutable but it has methods to mutate it

    // Cell RefCell and Mutex, mutex is in std::sync because it uses sincronization primitives provided by the OS or the CPU
    // Mutex is a kind of cell

    // They have different restrictions of what you can put inside them and how you can use them
    // the far you go towards Mutex the more flexibility but increases overhead

    //Cell, there's no way to get a reference to what's inside the cell, if you do a get, you'll get a copy
    //you can replace it, you can change it, and you can get a copy

    // it is always safe to mutate what's inside because no one can have a reference to the thing inside
    // cell does not implement Sync!
    // it you have a reference to a cell you can not give that reference to another thread

    // Cell allows you to have multiple references to a value, like in a graph and mutate it but only if it is happening in the
    // same thread
    // cell is usually used to small copy types

    // all is built with UnsafeCell
    // from share reference to a exclusive reference is not allow, the only way is through unsafe_cell

    // RefCell mutable memory location with dynamically checked borrow rules
    // it is useful when you know that you are going to need to mutate stuff but you're sure it is safe in runtime but it can't be proven in compile time

    DispatchQueue::serialize_async(async { Ok(true) });

    let addr = DispatchQueueWorker.start();

    // Send Ping message.
    // send() message returns Future object, that resolves to message result

    let ping = FutureMessage {
        async_task: do_async_task(),
    };

    let result = addr.send(ping).await.unwrap();

    match result {
        Ok(res) => println!("Got result: {}", res),
        Err(err) => println!("Got error: {}", err),
    }
}

async fn do_async_task() -> Result<bool, std::io::Error> {
    Ok(true)
}
