use actix::prelude::*;

#[derive(Debug)]
struct MyActor;

struct WhoAmI;

impl Message for WhoAmI {
    type Result = Result<actix::Addr<MyActor>, ()>;
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("started");
        ctx.set_mailbox_capacity(0);
    }
}

impl Handler<WhoAmI> for MyActor {
    type Result = Result<actix::Addr<MyActor>, ()>;

    fn handle(&mut self, msg: WhoAmI, ctx: &mut Self::Context) -> Self::Result {
        Ok(ctx.address())
    }
}

fn main() {
    let sys = System::new("sys");

    let addr = MyActor.start();

    let who_am_i_fut = addr.send(WhoAmI);

    Arbiter::spawn(
        who_am_i_fut
            .map(|res| {
                println!("{:?}", res);
            })
            .map_err(|_| ()),
    );

    sys.run();
}
