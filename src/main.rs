use std::env;


fn main() {
    
    // Get a handle on the arguments from the command line
    // args[1] would be the argument given to the application 
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);   
}

