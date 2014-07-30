#![feature(asm)]
fn main() {
	let a:i64;
	unsafe {
		asm!(
			"pop $0"
			:"=r"(a)
			::"memory":
			);
	};
	println!("{}",a);
}