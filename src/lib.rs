use std::error::Error;
use std::fs;
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
pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(config.file).expect("error reading msg");
    println!("{content}");
    Ok(content)
}
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
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
        assert_eq!(vec!["fast and productive"], search(query, content));
    }
}
