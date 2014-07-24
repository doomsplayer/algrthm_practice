#![crate_name = "sort"]
#![desc = "A Package about Sort Algorithms"]
#![license = "MIT"]
#![crate_type = "lib"]

use std::cmp::{Ord};
use std::result::Result;
use std::clone::Clone;
use std::result::{Ok};
use std::iter::range;

pub enum Order {
	Desc,
	Insc
}

pub fn insertion_sort<T:Ord+Clone>(v: &mut [T], ord: Order) -> Result<String,String> {
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


// fn merge_sort<T:Ord+Clone>(v:&mut [T],ord:Order)->Result<~str,~str>{
// 	let l = v.len();
// 	let mid = l/2;
// 	merge_sort(v.)
// 	if v.len()==2{

// 	}
// }

// fn merge<T:Ord+Clone>(v:&mut [T],ord:Order){

// }