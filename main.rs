extern crate matrix;

fn main() {
	let mut matrix = matrix::matrix::new(5,5);
	for i in range(1 as uint, 6 as uint) {
		for j in range(1 as uint, 6 as uint) {
			matrix.setValue((i, j), i+j);
		}
	}
	
	println!("{}",matrix.subMatrix((1,1), (5,5)));
	// println!("{}",matrix.subMatrix((1,2),(3,4)));
}