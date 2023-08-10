use std::future::Future;

use actix::ResponseFuture;
use actix::{Actor, Context, Handler, Message};

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct FutureMessage<T>
where
    T: Future<Output = Result<bool, std::io::Error>>,
{
    pub async_task: T,
}

// Define actor
pub struct DispatchQueueWorker;

// Provide Actor implementation for our actor
impl Actor for DispatchQueueWorker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}
    fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

/// Define handler for `FutureMessage` message
impl<T: 'static + Future<Output = Result<bool, std::io::Error>>> Handler<FutureMessage<T>>
    for DispatchQueueWorker
{
    //type Result = Box<dyn Future<Output = FutureMessageResponse>>;
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: FutureMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        Box::pin(async move {
            // Some async computation
            msg.async_task.await
        })
    }
}

// pub struct DispatchQueue;

// impl DispatchQueue {
//     async fn serialize_async(Addr<>) {}
// }
