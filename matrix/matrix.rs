extern crate std;

#[deriving(Eq)]
pub struct matrix<T> {
	line: uint,
	row: uint,
	content: ~[~[Option<T>]],
}

impl<T:Clone> matrix<T> {
	pub fn new(line: uint,row: uint) -> matrix<T> {
		matrix {content: {
			let mut a = ~[];
			for i in range(0, line) {
				a.push(~[]);
				for j in range(0, row) {
					a[i].push(None);
				}
			}
			a
		}, line: line, row: row}
	}

	// pub fn eye(n: uint, e: T) -> matrix<T> {
	// 	let mut ret = matrix::new(n, n);
	// 	for i in range (0, ret.line()) {
	// 		i.setValue	
	// 	}
		
	// }

	pub fn setValue(&mut self, (line, row): (uint, uint), value: T) {
		if self.line < line || self.row < row {
			fail!("index out of range");
		}
		self.content[line-1][row-1] = Some(value);
	}

	pub fn set(&mut self, (line, row): (uint, uint), value: Option<T>) {
		if self.line < line || self.row < row {
			fail!("index out of range");
		}
		self.content[line-1][row-1] = value;
	}

	pub fn line(&self) -> uint {
		self.line
	}

	pub fn row(&self) -> uint {
		self.row
	}

	pub fn get(&self, (line, row): (uint, uint)) -> Option<T> {
		self.content[line-1][row-1].clone()
	}

	pub fn subMatrix(&self, (fl, fr): (uint,uint), (tl, tr): (uint,uint)) -> matrix<T> {
		if fl >= tl {
			fail!("to line is smaller than from line");
		}
		if fr >= tr {
			fail!("to row is smaller than from row");
		}
		if fl < 0 || fr < 0 || tl < 0 || tr < 0 {
			fail!("point is smaller than 0")
		}
		matrix {
			content: {
				let mut ctnt = self.content.slice(fl - 1, tl).to_owned();
				for i in ctnt.mut_iter() {
					*i = i.slice(fr - 1, tr).to_owned()
				}
				ctnt
			},
			line: tl - fl + 1, 
			row: tr - fr + 1,
		}
	}
}

impl<T:std::fmt::Show> std::fmt::Show for matrix<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		try!(write!(f.buf,"\n"))
		for i in self.content.iter() {
			try!(write!(f.buf,"{}",i));
			try!(write!(f.buf,"\n"));
		}
		Ok(())
	}
}

