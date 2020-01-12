mod cli;
mod event;
mod ui;

#[macro_use]
extern crate lazy_static;
use gotham::state::State;
use std::sync::Arc;
use std::sync::Mutex;
use structopt::StructOpt;

lazy_static! {
    static ref event_cue: Arc<Mutex<Vec<event::Event>>> = Arc::new(Mutex::new(Vec::new()));
}

use druid::{AppLauncher, WindowDesc};
fn main() {
    let args = cli::Config::from_args();
    let addr = format!("127.0.0.1:{}", args.port);
    let main_window = WindowDesc::new(ui::imageview);

    println!("Listening on port {}", &addr);

    let data = ui::Image {
        width: 0,
        height: 0,
        data: vec![],
    };

    std::thread::spawn(|| start_server(addr));
    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("launch failed");
}

fn start_server(port: String) {
    let c = event_cue.clone();
    gotham::start(port, move || {
        let clone = c.clone();
        Ok(move |state: State| {
            let events = vec![event::Event {
                window: "Sample".into(),
                action: event::Action::Move { x: 300., y: 300. },
            }];
            let mut cue = clone.lock().unwrap();

            for event in events {
                cue.push(event);
            }

            (state, "Hello")
        })
    })
}
