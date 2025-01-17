use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub filename:String,
    pub query:String,
    pub case_sensitive:bool
}

impl Config {
    pub fn new(args:&[String])->Result<Config,&'static str>{
        if args.len() < 3{
           return Err("Not enough arguments")
        }
        let query = args[1].clone();
        let filename= args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
       Ok(Config{query,filename,case_sensitive})
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive{
        search(&config.query, &content)
    }else{
        search_case_insensitive(&config.query, &content)
    };
    for line in result{
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str) -> Vec<& 'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<& 'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod test{
    use super::*;


    #[test]
    fn case_sensitive() {
    let query = "duct";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuck tape";
    assert_eq!(
    vec!["safe, fast, productive."],
    search(query, contents)
    );
    }

    #[test]
    fn case_insensitive() {
    let query = "rUst";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me";
    assert_eq!(
    vec!["Rust:", "Trust me"],
    search_case_insensitive(query, contents)
    );
    }
}