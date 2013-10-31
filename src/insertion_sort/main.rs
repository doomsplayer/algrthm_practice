extern mod extra;
extern mod sorts(name = "sorts", vers = "0.1");
use std::rand::random;
use sort = sorts::insertion_sort;
use extra::time::get_time;
fn main(){
	let mut to_be_sort : ~[u16]= ~[];
	for _ in range(0,20){
		to_be_sort.push(random());
	}
	println!("The vector to be sorted is:\n{}.",to_be_sort.to_str());
	println!("Sort starts for {} elements.",to_be_sort.len());
	let st = get_time();
	sort(to_be_sort);
	let et = get_time();
	println!("Sort completed with time comsumption: {} seconds, {} nanoseconds.",et.sec-st.sec,et.nsec-st.nsec);
	println!("Result is here:\n{}.",to_be_sort.to_str());
}