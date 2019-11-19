pub struct Node{
	pub parent: *const i32,
	pub left: *const i32,
	pub right: *const i32,
	pub count: u32,
	pub data: char,
}
