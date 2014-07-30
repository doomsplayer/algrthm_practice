#![feature(asm)]

fn main() {asm();}

#[cfg(target_arch = "x86_64")]
fn asm() {
	use std::mem::transmute;
	use std::rand::random;

	
 	let array: &[u64] = &[random(),random(),random(),random()];

 	let address = unsafe { transmute::<_, (i64, i64)>(array).val0() };

 	for offset in range(0u,4) {
	 	let ret: u64;
	 	unsafe {
	 		asm!(
	 			r"
	 			mov ($1, $2, 8), %rax;
	 			mov %rax, $0;
	 			"
	 			: "=r"(ret)
	 			: "r"(address), "r"(offset)
	 			: "rax"
	 			:
	 			);
	 	}
	 	println!("在第{}号位上的元素是{}", offset, ret);
	 }
 }