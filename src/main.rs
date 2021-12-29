use std::env;
use std::fs;
use csv::Error;
use serde::Deserialize;



#[derive(Deserialize)]
struct Problem {
    question: String,
    answer: String,
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        Config { filename }
    }
}


fn get_problems(args: &[String]) -> Config {
    
    let filename = args[1].clone();
    
    Config{ filename }
}

fn main() -> Result<(), Error> {

    // Get a handle on the arguments from the command line
    // args[1] would be the argument given to the application 
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    let content = fs::read_to_string(config.filename)
        .expect("Something went wrong with accessing the file");

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(content.as_bytes());

    for result in reader.deserialize() {
        let record: Problem = result?;
        println!("{:?}, {:?}", record.question, record.answer);
    }
    

    
    

    Ok(())
}

