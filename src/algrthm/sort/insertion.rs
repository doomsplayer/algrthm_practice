use std::cmp::{Ord};
use std::result::Result;
use std::clone::Clone;
use std::result::{Ok};
use std::iter::range;
use super::{Order,Desc,Insc};

pub fn sort<T:Ord+Clone>(v: &mut [T], ord: Order) -> Result<String,String> {
	for j in range(1, v.len()) {
		let key = v[j].clone();
		let mut i = j as uint;
		match ord {
			Desc => {
					while i != 0 && v[i-1] < key {
						v[i] = v[i-1].clone();
						i -= 1;
					}
			}
			Insc => {
					while i != 0 && v[i-1] > key {
						v[i] = v[i-1].clone();
						i -= 1;
					}
			}
		}
		v[i] = key;	
	}
	Ok("ok".to_string())
}

