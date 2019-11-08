//creating a huffman tree struct to deal with the frequencies to encode contents
pub struct Tree{

    parent: *const i32,
    left: *const i32,
    right: *const i32,
    data: char,

}

pub fn build(tree: &mut Tree, freq: &mut [i32]){
    println!("entered build");//XXX this is filler code XXX
}

