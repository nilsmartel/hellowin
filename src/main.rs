#[macro_use]
extern crate lazy_static;
use gotham::state::State;
use std::sync::Arc;
use std::sync::Mutex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hellowin", about = "Create a controllable Window")]
struct Config {
    #[structopt(short = "p", long = "port", help = "Port to receive directions from")]
    port: String,
}

lazy_static! {
    static ref event_cue: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(Vec::new()));
}

fn main() {
    let args = Config::from_args();
    let addr = format!("127.0.0.1:{}", args.port);

    println!("Listening on port {}", &addr);

    start_server(addr);
}

fn start_server(port: String) {
    let c = event_cue.clone();
    gotham::start(port, move || {
        let clone = c.clone();
        Ok(move |state: State| {
            let events = vec![Event {
                window: "Sample".into(),
                action: Action::Move { x: 300., y: 300. },
            }];

            {
                let mut cue = clone.lock().unwrap();

                for event in events {
                    cue.push(event);
                }
            }

            (state, "Hello")
        })
    })
}

struct Event {
    window: String,
    action: Action,
}

enum Action {
    Resize { width: f64, height: f64 },
    Move { x: f64, y: f64 },
    Close,
}
