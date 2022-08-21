#![feature(int_log)]
#![feature(ptr_const_cast)]
#![feature(ptr_sub_ptr)]


mod class;
mod gc;

use gc::GarbageCollector;






fn main() {
	unsafe {
		let mut gc = GarbageCollector::new();

		let mut pointers: [*mut i64; 200] = [std::ptr::null_mut(); 200];

		for i in 1..200 {
			pointers[i] = gc.alloc(i);

			let slice = std::slice::from_raw_parts_mut(pointers[i], i);
			slice.fill(i as i64);
		}

		gc.begin_collect();
		gc.trace_ptr(pointers[1]);
		gc.trace_ptr(pointers[105]);
		gc.end_collect();
		println!("aaaaa");
	}
}
