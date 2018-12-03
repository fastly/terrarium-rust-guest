use hostcalls::raw;
use std::os::raw::c_void;

extern "C" fn default_malloc_impl(size: usize) -> *mut c_void {
    let v = vec![0u8; size].into_boxed_slice();
    Box::into_raw(v) as _
}

pub fn free(ptr: *mut c_void) {
    if !ptr.is_null() {
        let _ = unsafe { Box::from_raw(ptr) };
    }
}

extern "C" fn default_free_impl(ptr: *mut c_void) {
    free(ptr)
}

fn init_mm_raw(
    malloc_impl: extern "C" fn(size: usize) -> *mut c_void,
    free_impl: extern "C" fn(ptr: *mut c_void),
) {
    unsafe { raw::hostcall_init_mm(malloc_impl, free_impl) }
}

pub fn init_mm_default() {
    init_mm_raw(default_malloc_impl, default_free_impl)
}
