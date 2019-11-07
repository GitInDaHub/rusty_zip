mod comp;
mod decomp;

use std::io::self;
fn main() {
    
    //prompts user for filename through the command line
    println!("Please enter the name of the file to be zipped:");

    //creates a string that is to be used as the input buffer string 
    let mut buffer = String::new();

    //appends user input onto the string buffer
    io::stdin().read_line(& buffer).expect("Failed to get name of file to be zipped.");
    
    //removes excess characters from the string retrieved from user
    buffer = String::from(buffer.trim());

    if()
    else{
	comp(mut &buffer);
    }
}