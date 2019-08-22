use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;

enum Msg {
    Ping,
    Pong,
}

enum Resp {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Resp
where
    A: Actor,
    M: Message<Result = Resp>,
{
    fn handle<R: ResponseChannel<M>>(self, ctx: &mut <A as Actor>::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl Message for Msg {
    type Result = Resp;
}

struct PingActor;

impl Actor for PingActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor is stopped");
    }
}

impl Handler<Msg> for PingActor {
    type Result = Resp;

    fn handle(&mut self, msg: Msg, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Msg::Ping => Resp::GotPing,
            Msg::Pong => Resp::GotPong,
        }
    }
}

fn main() {
    let sys = System::new("example");

    // Start PingActor in current thread
    let addr = PingActor.start();

    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let ping_fut = addr.send(Msg::Ping);
    let pong_fut = addr.send(Msg::Pong);

    // Spawn pong_future onto event loop
    Arbiter::spawn(
        pong_fut
            .map(|res| match res {
                Resp::GotPing => println!("Ping received"),
                Resp::GotPong => println!("Pong received"),
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    // Spawn ping_future onto event loop
    Arbiter::spawn(
        ping_fut
            .map(|res| match res {
                Resp::GotPing => println!("Ping received"),
                Resp::GotPong => println!("Pong received"),
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    sys.run();
}
