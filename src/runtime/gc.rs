use bdwgc_sys as bdwgc;
use std::{
	mem::{size_of},
	ops::{Deref},
	slice
};


pub struct GcRef<T: ?Sized> {
	_ptr: *const T
}

impl<T: ?Sized> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
		unsafe {
			return self._ptr.as_ref().unwrap_unchecked()
		}
    }
}


pub struct GarbageCollector {

}

impl GarbageCollector {
	#[inline(always)]
	pub(crate) fn new() -> Self {
		unsafe {
			bdwgc::GC_init();
			return Self {  }
		}
	}

	#[inline(always)]
	pub fn alloc<T>(&mut self, value: T, is_static: bool, is_atomic: bool) -> GcRef<T> {
		unsafe {
			let ptr = self._alloc::<T>(1, is_static, is_atomic);
			ptr.write(value);
			return GcRef { _ptr: ptr};
		}
	}

	#[inline(always)]
	pub fn alloc_slice<T: Clone>(&mut self, values: &[T], is_static: bool, is_atomic: bool) -> GcRef<[T]> {
		unsafe {
			let ptr = self._alloc::<T>(values.len(), is_static, is_atomic);
			
			for (i, v) in values.iter().enumerate() {
				ptr.offset(i as isize).write(v.clone())
			}
			let ptr = slice::from_raw_parts(ptr, values.len());

			return GcRef { _ptr: ptr};
		}
	}
	
	#[inline(always)]
	pub fn dealloc<T: ?Sized>(&mut self, ptr: GcRef<T>) {
		unsafe {
			self._dealloc(ptr._ptr.to_raw_parts().0);
		}
	}
}

impl GarbageCollector {
	#[inline(always)]
	unsafe fn _alloc<T>(&mut self, count: usize, is_static: bool, is_atomic: bool) -> *mut T {
		if is_static {
			if is_atomic { return bdwgc::GC_malloc_atomic_uncollectable((size_of::<T>() * count).try_into().unwrap_unchecked()).cast(); }
			return bdwgc::GC_malloc((size_of::<T>() * count).try_into().unwrap_unchecked()).cast();
		} else {
			if is_atomic { return bdwgc::GC_malloc_atomic_uncollectable((size_of::<T>() * count).try_into().unwrap_unchecked()).cast(); }
			return bdwgc::GC_malloc((size_of::<T>() * count).try_into().unwrap_unchecked()).cast();
		}
	}

	#[inline(always)]
	unsafe fn _dealloc<T>(&mut self, ptr: *const T) {
		bdwgc::GC_free(ptr.as_mut().cast());
	}
}
