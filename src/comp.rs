use crate::h_tree::*;
use priority_queue::PriorityQueue;
use std::fs::File;
use std::io::{Read, Write};
use std::vec;
use std::clone;
use std::iter::IntoIterator;

const ASCII: usize = 256;
/* This method takes in a file and compresses its contents into a smaller file and then does
 * a lossless hash on it
 * The method takes in a String of the file to be compressed
 */
pub fn rzip(in_file_name: &mut String){
    //XXX deal with compressing the files content
    //opening file to be compressed
    let mut in_file = File::open(in_file_name).expect("in_File not opened.");

    //creating a string to hold the contents of the file
    let mut contents = String::new();

    //reads in the contents of the file onto the string contents
    in_file.read_to_string(&mut contents).expect("File not read.");


    // Creating an array to store the frequency of each character in the file
    let mut frequency: [i32; ASCII] = [0; ASCII];

    //creates a vector made out of the string from contents
    let char_vect: Vec<char> = contents.chars().collect();

    //loops through the characters found in the file and fills in the frequency array
    for &x in &char_vect{
	frequency[x as usize] = frequency[x as usize] + 1;
    }
    let mut temp = PriorityQueue::new();
    //creating huffman tree obejct XXX create node objects and pass pointers
    let mut tree = crate::h_tree::Tree {
	root: 0 as *const i32,
	leaves: &*temp,
    };
    //creates a new string for the out_file_name
    let mut file_suffix = String::from(".zip");
    let mut out_file_name = in_file_name.clone();
    out_file_name = [out_file_name, file_suffix].join("");
 
    //builds the huffman tree
    crate::h_tree::build(&mut tree, &mut frequency);
    /* XXX Testing to see if frequenchy array filled in correctly XXX
    for &y in frequency.iter(){
	println!("{}",y);
    } XXX */
}
