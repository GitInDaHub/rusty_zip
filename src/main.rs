mod comp;
mod decomp;
mod h_tree;
mod h_node;

use std::io;


fn main() {
    
    //prompts user for filename through the command line
    println!("Please enter the name of the file to be zipped:");

    //creates a string that is to be used as the input buffer string 
    let mut buffer_in = String::new();
    
    //appends user input onto the string buffer
    io::stdin().read_line(&mut buffer_in).expect("Failed to get name of file to be zipped.");
    
    //removes excess characters from the string retrieved from user
    buffer_in = String::from(buffer_in.trim());

    // Checks whether or not file entered is already zipped or unzipped and calls
    // appropriate method according to the file entered
    if buffer_in.ends_with(".rzip") {
	decomp::runzip(&mut buffer_in);
    }else{
	comp::rzip(&mut buffer_in);
    }
}
