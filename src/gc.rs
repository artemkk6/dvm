use std::collections::VecDeque;
use std::mem::{size_of, transmute};
use std::alloc::{alloc, dealloc, Layout};




pub struct GcBox<T> {
	_ptr: *mut T
}


impl<T> AsRef<T> for GcBox<T> {
    fn as_ref(&self) -> &T {
		unsafe {
			return self._ptr.as_ref().unwrap_unchecked();
		} 
    }
}

impl<T> std::borrow::Borrow<T> for GcBox<T> {
    fn borrow(&self) -> &T {
		unsafe {
			return &**self;
		} 
    }
}

impl<T> std::ops::Deref<T> for GcBox<T>

pub struct GarbageCollector {
	_used_blocks: Vec<Block>,
	_free_blocks: Vec<Block>,
	_marked_objects: Vec<*mut u8>,
	_block_size: usize
	
}


impl GarbageCollector {
	fn new(block_size: usize) -> Self {
		let mut gc = Self {
			_used_blocks: Vec::with_capacity(256),
			_free_blocks: Vec::with_capacity(256),
			_marked_objects: Vec::with_capacity(0x10_000),
			_block_size: block_size
		};
		gc._used_blocks.push(
			unsafe {
				Block::new(block_size)
			}
		);

		return gc;
	}

	fn alloc<T>(&mut self, value: T) -> GcBox<T> {
		unsafe {
			let size = size_of::<T>();
			let ptr = self._used_blocks.last_mut().unwrap_unchecked()
				.alloc(size).unwrap_or_else(
					|| {
						if size > self._block_size { self._used_blocks.push(Block::new(size)); }
						self._used_blocks.last_mut().unwrap_unchecked().alloc(size).unwrap_unchecked()
					}
				);
			return transmute(ptr);
		}
	}

	fn mark<T>(&mut self, object: &GcBox<T>) {
		unsafe {
			self._marked_objects.push(
				transmute(object._ptr)
			);
		}
	}
}


struct Block {
	_begin: *mut u8,
	_end: *mut u8,
	_last: *mut u8
}

impl Block {
	pub unsafe fn new(size: usize) -> Self {
		let ptr = alloc(Layout::from_size_align_unchecked(size, 0x10));

		return Self {
			_begin: ptr,
			_end: ptr,
			_last: ptr.offset(size as isize)
		}
	}

	unsafe fn alloc(&mut self, size: usize) -> Option<*mut u8> {
		let ptr = self._last;
		self._last = self._last.offset(size as isize);

		if self._last > self._end { return None; }
		return Some(ptr);
	}

	fn reset(&mut self) {
		self._last = self._begin;
	}
}

impl Block {
	fn begin(&mut self) -> *mut u8 {
		return self._begin;
	}

	fn end(&mut self) -> *mut u8 {
		return self._end;
	}
}