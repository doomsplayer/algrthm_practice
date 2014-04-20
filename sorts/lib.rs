#![crate_id = "sort_algorithms#0.1"]
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
pub fn insertion_sort<T:Ord+Clone>(v: &mut [T],ord: Order) -> Result<~str,~str>{
	match ord{
		Desc => {
			for j in range(1, v.len()) {
				let key = v[j].clone();
				let mut i = (j-1) as int;
				while i >= 0 && v[i]<key {
					v[i+1]=v[i].clone();
					i-=1;
				}
				v[i+1] = key;
			};
			Ok(~"ok")
		}
		Insc => {
			for j in range(1,v.len()){
				let key = v[j].clone();
				let mut i = (j-1) as int;
				while i >= 0 && v[i]>key{
					v[i+1]=v[i].clone();
					i-=1;
				}
				v[i+1] = key;
			};
			Ok(~"ok")
		}
	}	
}

fn strassen_multiply(lhs: &[&[int]], rhs: &[&[int]]) -> Option<&[&[int]]> {
	fn submatrix(i: int, j: int) ->
	let len = lhs.len();
	if lhs.len() != len || rhs.len() != len {
		return None
	}
	for i in range(0, lhs.len()){
		if lhs[i].len() != len || rhs[i].len() != len {
			return None
		}
	}

	match len {
		1 => {
			lhs[0][0] * rhs[0][0]
		}
		n if n % 2 == 0 => {
			strassen_multiply()
		}
		n if n % 2 == 1 => {

		}
	}
	None
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