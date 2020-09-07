use crate::piece::{
	PieceAdjacency,
	PieceColor,
};

pub struct Adjacency<T>([T; 4]);

impl<T> Adjacency<T> {
	pub fn new(up: T, down: T, left: T, right: T) -> Self {
		Self([up, down, left, right])
	}

	fn get(&self, i: usize) -> &T {
		&self.0[i]
	}

	pub fn up(&self) -> &T {
		self.get(0)
	}
	pub fn down(&self) -> &T {
		self.get(1)
	}
	pub fn left(&self) -> &T {
		self.get(2)
	}
	pub fn right(&self) -> &T {
		self.get(3)
	}
	pub fn iter(&self) -> std::slice::Iter<'_, T> {
		self.0.iter()
	}
	pub fn to_array(self) -> [T; 4] {
		self.0
	}
	pub fn map<U, F: Fn(&T) -> U>(self, func: F) -> Adjacency<U> {
		Adjacency::new(
			func(&self.0[0]),
			func(&self.0[1]),
			func(&self.0[2]),
			func(&self.0[3]),
		)
	}
}
