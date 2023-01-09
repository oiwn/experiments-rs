use actix::prelude::*;

/// Define `Ping` message
struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

/// Actor
struct MyActor {
    count: usize,
}

/// Declare actor and its context
impl Actor for MyActor {
    type Context = Context<Self>;
}

/// Handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

pub async fn run() {
    let addr = MyActor { count: 10 }.start();
    let res = addr.send(Ping(10)).await;

    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap() == 20);
}
