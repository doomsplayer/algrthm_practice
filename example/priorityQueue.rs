extern crate debug;
use std::collections::PriorityQueue;
use std::rand::random;

fn main() {
	let mut pq = PriorityQueue::<int>::new();
	for _ in range(0i,10) {
		let rd = random();
		println!("pushed {} into pq", rd);
		pq.push(rd);
	}
	for _ in range(0i,10) {
		println!("{}", pq.top());	
	}
	

}