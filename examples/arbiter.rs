use actix::prelude::*;

struct SumActor;

impl Actor for SumActor {
    type Context = Context<Self>;
}

struct Value(usize, usize);

impl Message for Value {
    type Result = usize;
}

impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, ctx: &mut Self::Context) -> Self::Result {
        msg.0 + msg.1
    }
}

struct DisplayActor;

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

struct Display(usize);

impl Message for Display {
    type Result = ();
}

impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: Display, ctx: &mut Self::Context) -> Self::Result {
        println!("Got {:?}", msg.0);
    }
}

fn main() {
    let system = System::new("single-arbiter-system");

    // `Actor::start` spawns the `Actor` on the *current* `Arbiter`, which
    // in this case is the System arbiter
    let sum_addr = SumActor.start();
    let dis_addr = DisplayActor.start();

    // Define an execution flow using futures
    //
    // Start by sending a `Value(6, 7)` to our `SumActor`.
    // `Addr::send` responds with a `Request`, which implements `Future`.
    // When awaited or mapped, it will resolve to a `Result<usize, MailboxError>`.
    let execution = sum_addr
        .send(Value(6, 7))
        // `.map_err` turns `Future<usize, MailboxError>` into `Future<usize, ()>`
        //   and prints any `MailboxError`s we encounter
        .map_err(|e| {
            eprintln!("Mailbox error: {:?}", e);
        })
        // Assuming the send was successful, chain another computation
        //   onto the future. Returning a future from `and_then` chains
        //   that computation to the end of the existing future.
        .and_then(move |res| {
            // `res` is now the `usize` returned from `SumActor` as a response to `Value(6, 7)`

            // Once the future is complete, send the successful response (`usize`)
            // to the `DisplayActor` wrapped in a `Display
            dis_addr.send(Display(res)).map(|_|()).map_err(|_|())
        })
        .map(|_| {
            // We only want to do one computation in this example, so we
            // shut down the `System` which will stop any Arbiters within
            // it (including the System Arbiter), which will in turn stop
            // any Actor Contexts running within those Arbiters, finally
            // shutting down all Actors.
            System::current().stop();
        });

    Arbiter::spawn(execution);

    system.run();
}
