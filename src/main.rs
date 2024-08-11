use echo_rs::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let echoes = &args[1..];
    run(echoes);
}
