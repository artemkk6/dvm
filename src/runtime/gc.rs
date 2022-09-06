use bdwgc_sys as bdwgc;
use std::mem::{size_of};


#[inline(always)]
pub(crate) unsafe fn gc_init() {
	bdwgc::GC_init();
	bdwgc::GC_enable_incremental();
}

#[inline(always)]
pub(crate) unsafe fn alloc<T>(count: usize, atomic: bool) -> *mut T {
	if atomic { return bdwgc::GC_malloc_atomic_uncollectable((size_of::<T>() * count).try_into().unwrap_unchecked()).cast(); }
	return bdwgc::GC_malloc((size_of::<T>() * count).try_into().unwrap_unchecked()).cast();
}

#[inline(always)]
pub(crate) unsafe fn alloc_uncollectable<T>(count: usize, atomic: bool) -> *mut T {
	if atomic { return bdwgc::GC_malloc_atomic_uncollectable((size_of::<T>() * count).try_into().unwrap_unchecked()).cast(); }
	return bdwgc::GC_malloc((size_of::<T>() * count).try_into().unwrap_unchecked()).cast();
}

#[inline(always)]
pub(crate) unsafe fn dealloc<T>(ptr: *mut T) {
	bdwgc::GC_free(ptr.cast());
}