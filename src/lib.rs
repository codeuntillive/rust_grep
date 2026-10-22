use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process as Process;
pub struct Config {
    pub query: String,
    pub file: String,
}
impl Config {
    pub fn set_config(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            panic!("two argument req");
        }
        let query: String = args[1].clone();
        let file: String = args[2].clone();
        Ok(Self { query, file })
    }
}
pub fn run<'a>(config: Config) -> Result<u8, Box<dyn Error>> {
    let content = fs::read_to_string(config.file).expect("error reading msg");

    let result = search(&config.query, &content);
    if result.0.len() == 0 {
        eprintln!(
            "\x1b[31mError: Query '{}' not found in file.\x1b[0m",
            config.query
        );
        Process::exit(1);
    }
    let map: HashMap<_, _> = result.0.clone().into_iter().zip(result.1.clone()).collect();
    for (i, n) in &map {
        println!("\x1b[1;32mFound\x1b[0m :-> \"{i}\"       line number :{n}");
    }

    Ok(1 as u8)
}
pub fn search<'a>(query: &str, content: &'a str) -> (Vec<&'a str>, Vec<usize>) {
    let mut results = Vec::new();
    let mut linenum = Vec::<usize>::new();
    for (num, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            linenum.push(num);
            results.push(line.trim());
        }
    }

    (results, linenum)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result() {
        let query: &str = "fast and productive";
        let content: &str = "\
            fdfddd
            ddcvdvv du
            csszcz sdfvd g
             ghdg df
              dfd
              fast and productive
            ";
        assert_eq!(vec!["fast and productive"], search(query, content).0);
    }
}
