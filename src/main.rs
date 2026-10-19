use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file: String,
}
impl Config {
    fn set_config(args: &[String]) -> Result<Self, &str> {
        if args.len < 3 {
            panic!("two argument req");
        }
        let query: String = args[1].clone();
        let file: String = args[2].clone();
        Ok(Self { query, file })
    }
}
fn main() {
    println!("mini grep");
    let argss: Vec<String> = env::args().collect();
    println!("{:?}", argss);
    let config: Config = Config::set_config(&argss).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(0)
    });
    let content = fs::read_to_string(config.file).expect("error reading msg");
    println!("{content}");
}
