use std::env;
use std::fs;

struct Config {
    query: String,
    file: String,
}
fn set_config(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let file: String = args[2].clone();
    Config { query, file }
}
fn main() {
    println!("mini grep");
    let argss: Vec<String> = env::args().collect();
    println!("{:?}", argss);
    let config: Config = set_config(&argss);
    let content = fs::read_to_string(config.file).expect("error reading msg");
    println!("{content}");
}
