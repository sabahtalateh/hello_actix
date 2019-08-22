use actix::prelude::*;

#[derive(Message)]
struct Signal(usize);

/// Subscribe to process signals.
#[derive(Message)]
struct Subscribe(pub Recipient<Signal>);

/// Actor that provides signal subscriptions
#[derive(Default)]
struct ProcessSignals {
    subscribers: Vec<Recipient<Signal>>,
}

impl Actor for ProcessSignals {
    type Context = Context<Self>;
}

impl ProcessSignals {
    /// Send signal to all subscribers
    fn send_signal(&mut self, sig: usize) {
        for subsc in &self.subscribers {
            subsc.do_send(Signal(sig));
        }
    }
}

impl Handler<Subscribe> for ProcessSignals {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, ctx: &mut Self::Context) -> Self::Result {
        self.subscribers.push(msg.0);
    }
}

fn main() {
    let sys = System::new("sys");

    let sig_five = Signal(5);

    let signals_processor = ProcessSignals::default();
//    let signals_processor_actor = signals_processor.start();
//    let ff = signals_processor_actor.recipient();
//    let hhh = signals_processor_actor.send(Subscribe(Recipient(sig_five)));

    let addr = ProcessSignals::create(|ctx| {
        let addr = ctx.address();
        let mut recipients = vec![];
        recipients.push(addr.recipient());
        ProcessSignals{
            subscribers: recipients,
        }
    });


    sys.run();
}
