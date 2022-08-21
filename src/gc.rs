use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, transmute};


const BLOCK_SIZE: usize = 0x1000;


pub unsafe trait Trace {
	unsafe fn trace(&self, gc: &mut GarbageCollector);
}

pub struct GcRef<T: Trace> {
	_ptr: *const T
}

pub struct GcRefMut<T: Trace> {
	_ptr: *mut T
}


pub struct GarbageCollector {
	_used_blocks: Vec<Block>,
	_free_blocks: Vec<Block>,
	_marked_pointers: Vec<*mut u8>
}

struct Block {
	_begin: *mut u8,
	_end: *mut u8,
	_last: *mut u8,
	_need_reset: bool
}



impl Block {
	fn new(size: usize) -> Block {
		unsafe {
			let ptr = alloc(Layout::from_size_align_unchecked(size, size));

			return Block {
				_begin: ptr,
				_end: ptr.offset(size as isize),
				_last: ptr,
				_need_reset: false
			};
		}
	}

	#[inline(always)]
	unsafe fn alloc(&mut self, size: usize) -> Option<*mut u8> {
		let ptr = self._last;
		self._last = self._last.offset(size as isize);

		if self._last > self._end { return None; }
		return Some(ptr.cast());
	}

	#[inline(always)]
	fn contains(&self, ptr: *const u8) -> bool {
		return self._begin.as_const() <= ptr && ptr < self._end.as_const();
	}

	fn free(&mut self) {
		self._need_reset = false;
		self._last = self._begin;
	}

	unsafe fn drop(&self) {
		let size = self._end.sub_ptr(self._begin);
		dealloc(self._begin, Layout::from_size_align_unchecked(size, size));
	}
}

impl GarbageCollector {
	pub(crate) fn new() -> GarbageCollector {
		let mut gc = GarbageCollector {
			_used_blocks: Vec::with_capacity(0x100),
			_free_blocks: Vec::with_capacity(0x100),
			_marked_pointers: Vec::with_capacity(0x1000)
		};
		gc._used_blocks.push(Block::new(BLOCK_SIZE));

		return gc;
	}

	pub(crate) unsafe fn alloc<T>(&mut self, count: usize) -> *mut T {
		let size = size_of::<T>() * count;

		let ptr = self._used_blocks.last_mut().unwrap_unchecked().alloc(size);
		if let Some(ptr) = ptr { return ptr.cast(); }

		if size > BLOCK_SIZE {
			let block_size = 1 << (size.log2() + 1);
			self._used_blocks.push(Block::new(block_size));
		} else {
			self._used_blocks.push(
				self._free_blocks.pop().unwrap_or(
					Block::new(BLOCK_SIZE)
				)
			);
		}

		return self._used_blocks.last_mut().unwrap_unchecked()
			.alloc(size).unwrap_unchecked().cast();
	}

	pub(crate) fn begin_collect(&mut self) {
		for block in &mut self._used_blocks {
			block._need_reset = true;
		}
		self._free_blocks.append(&mut self._used_blocks);
	}

	pub(crate) fn end_collect(&mut self) {
		self._free_blocks.sort_by(|a, b| a._begin.cmp(&b._begin));
		self._marked_pointers.sort();

		unsafe {
			let mut blocks: Vec<Block> = Vec::with_capacity(self._free_blocks.capacity());
			std::mem::swap(&mut blocks, &mut self._free_blocks);
			
			let mut block = blocks.as_mut_ptr();
			let end_block = block.offset(blocks.len() as isize);

			for &ptr in &self._marked_pointers {
				while (*block)._end <= ptr {
					(*block).free();
					self._free_blocks.push(block.read());
					block = block.offset(1);
					if block >= end_block { break; }
				}

				if (*block).contains(ptr) {
					self._used_blocks.push(block.read());
					block = block.offset(1);
					if block >= end_block { break; }
				}
			}

			while block <= end_block {
				(*block).free();
				self._free_blocks.push(block.read());
				block = block.offset(1);
			}
		}
		
		self._marked_pointers.clear()
	}
}

impl GarbageCollector {
	#[inline(always)]
	pub unsafe fn trace<T: Trace>(&mut self, object: &T) {
		object.trace(self);
	}

	#[inline(always)]
	pub unsafe fn trace_ptr<T>(&mut self, ptr: *const T) {
		self._marked_pointers.push(
			transmute(ptr)
		);
	}
}


unsafe impl<T: Trace> Trace for GcRef<T> {
	#[inline(always)]
    unsafe fn trace(&self, gc: &mut GarbageCollector) {
        gc.trace_ptr(self._ptr);
		(*self._ptr).trace(gc);
    }
}

impl<T: Trace> std::ops::Deref for GcRef<T> {
    type Target = T;

	#[inline(always)]
    fn deref(&self) -> &Self::Target {
		unsafe {
			return self._ptr.as_ref().unwrap_unchecked()
		}
    }
}


unsafe impl<T: Trace> Trace for GcRefMut<T> {
	#[inline(always)]
    unsafe fn trace(&self, gc: &mut GarbageCollector) {
        gc.trace_ptr(self._ptr);
		(*self._ptr).trace(gc);
    }
}

impl<T: Trace> std::ops::Deref for GcRefMut<T> {
    type Target = T;

	#[inline(always)]
    fn deref(&self) -> &Self::Target {
		unsafe {
			return self._ptr.as_ref().unwrap_unchecked()
		}
    }
}






