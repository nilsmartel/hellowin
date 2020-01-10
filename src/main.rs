use structopt::StructOpt;

use gotham::state::State;

#[derive(Debug, StructOpt)]
#[structopt(name = "hellowin", about = "Create a controllable Window")]
struct Config {
    #[structopt(short = "p", long = "port", help = "Port to receive directions from")]
    port: String,
}

fn main() {
    let args = Config::from_args();
    let addr = format!("127.0.0.1:{}", args.port);

    println!("Listening on port {}", &addr);

    gotham::start(addr, || Ok(handler_function))
}

fn handler_function(state: State) -> (State, &'static str) {
    (state, "Hello")
}
