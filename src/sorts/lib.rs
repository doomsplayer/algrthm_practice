#[link(name = "sorts", vers = "0.1")];
#[author = "xLii"];
#[desc = "A Package about Sort Algorithms"];
#[license = "MIT"];
#[crate_type = "lib"];

use std::cmp::{Ord};
use std::result::Result;
use std::clone::Clone;
use std::result::{Ok};
use std::iter::range;

pub fn insertion_sort<T:Ord+Clone>(v:&mut [T])->Result<~str,~str>{
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