use rgrep::Config;
use rgrep::run;

use std::env;

use std::process;

fn main() {
    println!("mini grep");
    let argss: Vec<String> = env::args().collect();
    println!("{:?}", argss);
    let config: Config = Config::set_config(&argss).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(0)
    });
    if let Err(e) = run(config) {
        println!("{e}");
        process::exit(1);
    }
}
