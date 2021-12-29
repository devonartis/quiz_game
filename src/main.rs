use std::env;
use std::fs;
use csv::Error;
use serde::Deserialize;



#[derive(Deserialize)]
struct Problem {
    question: String,
    answer: String,
}


fn parse_args(args: &[String]) -> &str {
    
    let filename = &args[1];
    
    filename
}
fn main() -> Result<(), Error> {

    // Get a handle on the arguments from the command line
    // args[1] would be the argument given to the application 
    let args: Vec<String> = env::args().collect();
    
    let csv_filename = parse_args(&args);

    let content = fs::read_to_string(csv_filename)
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

