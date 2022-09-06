#![feature(ptr_const_cast)]
#![feature(ptr_metadata)]



mod runtime;


use runtime::gc::GarbageCollector;



fn main() {
	let mut gc = GarbageCollector::new();

	let mut data = [1, 2, 3, 4];
	let a = gc.alloc_slice(&data, false, false);
	data[1] = 10;
	let d = &*a;
	println!("{:?}, {:?}", d, data);
}
