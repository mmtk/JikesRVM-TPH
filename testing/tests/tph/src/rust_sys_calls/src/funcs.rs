use libc::{c_char, c_void};

#[no_mangle]
pub extern "C" fn test_stack_alignment_0() {
    println!("Entering stack alignment test with no args passed");
    unsafe {
        llvm_asm!("movaps %xmm1, (%esp)" : : : "sp", "%xmm1", "memory");
    }
    println!("Exiting stack alignment test");
}

#[allow(clippy::many_single_char_names)]
#[no_mangle]
pub extern "C" fn test_stack_alignment_5(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
) -> usize {
    println!("Entering stack alignment test");
    println!("a:{}, b:{}, c:{}, d:{}, e:{}", a, b, c, d, e);
    unsafe {
        llvm_asm!("movaps %xmm1, (%esp)" : : : "sp", "%xmm1", "memory");
    }
    let result = a + b * 2 + c * 3 + d * 4 + e * 5;
    println!("Exiting stack alignment test");
    result
}

#[no_mangle]
pub extern "C" fn tph_gc_init(_jtoc: *mut c_void, _heap_size: usize) {
    unreachable!();
}

#[no_mangle]
pub extern "C" fn tph_bind_mutator(_tls: *mut c_void) {
    unreachable!();
}

#[no_mangle]
pub extern "C" fn tph_process(_name: *const c_char, _value: *const c_char) {
    unreachable!();
}

#[no_mangle]
pub extern "C" fn tph_start_worker(_tls: *mut c_void, _worker: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_enable_collection(_tls: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_start_control_collector(_tls: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_will_never_move(_object: *mut c_void) -> i32 {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_free_bytes() -> usize {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_total_bytes() -> usize {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_is_mapped_object(_reference: *mut c_void) -> i32 {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_add_weak_candidate(_reference: *mut c_void, _referent: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_add_soft_candidate(_reference: *mut c_void, _referent: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_add_phantom_candidate(_reference: *mut c_void, _referent: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_add_finalizer(_object: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_get_finalized_object() -> *mut c_void {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_modify_check(_reference: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_handle_user_collection_request(_tls: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_harness_begin(_tls: *mut c_void) {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn tph_harness_end() {
    unreachable!()
}
