use actix::prelude::*;
use std::io;

// Define message
struct Ping;

impl Message for Ping {
    type Result = Result<bool, io::Error>;
}

// Define actor
struct PingActor;

impl Actor for PingActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor is stoped");
    }
}

impl Handler<Ping> for PingActor {
    type Result = Result<bool, io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received");

        Ok(true)
    }
}

fn main() {
    let sys = System::new("example");

    let actor = PingActor;

    // Start MyActor in current thread
    let addr = actor.start();

    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let result = addr.send(Ping);

    Arbiter::spawn(
        result
            .map(|res| match res {
                Ok(result) => println!("Got result: {}", result),
                Err(e) => println!("Got error: {}", e),
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    sys.run();
}
