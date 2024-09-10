use echo_rs::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let echo_str = args[1..].join(" ");
    run(&echo_str);
}
