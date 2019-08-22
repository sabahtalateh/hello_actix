use actix::prelude::*;

struct PingActor {
    count: usize,
}

impl Actor for PingActor {
    type Context = Context<Self>;
}

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

impl Handler<Ping> for PingActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        println!("handle");
        self.count += msg.0;

        self.count
    }
}

fn main() -> std::io::Result<()> {
    let system = System::new("test");

    // start new actor
    let addr = PingActor { count: 10 }.start();

    // send a message and get a future for result
    let res = addr.send(Ping(20));

    Arbiter::spawn(
        res.map(|res| {
            println!("RESULT: {}", res == 30);
        })
        .map_err(|_| ()),
    );

    system.run()
}
