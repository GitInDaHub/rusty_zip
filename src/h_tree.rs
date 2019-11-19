use crate::h_node::*;
use priority_queue::PriorityQueue;

//creating a huffman tree struct to deal with the frequencies to encode contents
pub struct Tree{
    
    pub root: *const i32,
    pub leaves: PriorityQueue,

}

pub fn build(tree: &mut Tree, freq: &mut [i32]){
    println!("entered build");//XXX this is filler code XXX
}

