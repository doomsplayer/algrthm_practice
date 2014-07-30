extern crate algorithm;
extern crate time;
use std::rand::random;
use algorithm::sort::{Desc,Insc};
use algorithm::sort::insertion::sort;
use time::get_time;

fn main(){
	let mut to_be_sort : Vec<u16> = Vec::new();
	for _ in range(0i, 20){
		to_be_sort.push(random());
	}
	println!("The vector to be sorted is:\n{}.",to_be_sort);
	println!("Sort starts for {} elements.",to_be_sort.len());
	println!("Desc");
	let st = get_time();
	let _ = sort(to_be_sort.as_mut_slice(), Desc);
	let et = get_time();
	println!("Sort completed with time comsumption: {} seconds, {} nanoseconds.",et.sec-st.sec,et.nsec-st.nsec);
	println!("Result is here:\n{}.",to_be_sort);

	println!("Insc");
	let st = get_time();
	let _ = sort(to_be_sort.as_mut_slice(), Insc);
	let et = get_time();
	println!("Sort completed with time comsumption: {} seconds, {} nanoseconds.",et.sec-st.sec,et.nsec-st.nsec);
	println!("Result is here:\n{}.",to_be_sort);
}

