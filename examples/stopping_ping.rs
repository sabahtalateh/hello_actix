use actix::prelude::*;

struct PingActor {
    count: usize,
}

impl Actor for PingActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("stopped");
    }
}

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

impl Handler<Ping> for PingActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        println!("handle {} + {}", self.count, msg.0);

        self.count += msg.0;

        if self.count > 5 {
            println!("shutting down ping receiver");
            ctx.stop()
        }

        self.count
    }
}

fn main() -> std::io::Result<()> {
    let system = System::new("test");

    // start new actor
    let addr = PingActor { count: 0 }.start();
    let addr_2 = addr.clone();
    let res = addr.send(Ping(6));

    Arbiter::spawn(
        res.map(move |res| {
            // Now, the ping actor should have stopped, so a second message will fail
            // With a SendError::Closed
            assert!(addr_2.try_send(Ping(1)).is_err());

            // Shutdown gracefully now
            System::current().stop();
        })
        .map_err(|_| ()),
    );

    system.run()
}
