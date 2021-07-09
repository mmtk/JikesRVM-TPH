#include "sys.h"

EXTERNAL void test_stack_alignment_0();
EXTERNAL void test_stack_alignment_5(int a, int b, int c, int d, int e);

// #ifdef TPH_BUILD

EXTERNAL void tphGCInit(void* jtoc, size_t heap_size) {
    tph_gc_init(jtoc, heap_size);
}

EXTERNAL void* tphBindMutator(size_t thread_id) {
    return tph_bind_mutator(thread_id);
}

EXTERNAL bool tphProcess(char* name, char* value) {
    return tph_process(name, value);
}

EXTERNAL void tphStartWorker(void *tls, void* worker) {
    tph_start_worker(tls, worker);
}

EXTERNAL void tphEbableCollection(void* tls) {
    tph_enable_collection(tls);
}

EXTERNAL void tphStartControlCollector(void* tls) {
    tph_start_control_collector(tls);
}

EXTERNAL bool tphWillNeverMove(void* object) {
    return tph_will_never_move(object);
}

EXTERNAL size_t tphFreeBytes() {
    return tph_free_bytes();
}

EXTERNAL size_t tphTotalBytes() {
    return tph_total_bytes();
}

EXTERNAL bool tphIsMappedObject(void* ref) {
    return tph_is_mapped_object(ref);
}

EXTERNAL void tphAddWeakCandidate(void* ref, void* referent) {
    tph_add_weak_candidate(ref, referent);
}

EXTERNAL void tphAddSoftCandidate(void* ref, void* referent) {
    tph_add_soft_candidate(ref, referent);
}

EXTERNAL void tphAddPhantomCandidate(void* ref, void* referent) {
    tph_add_phantom_candidate(ref, referent);
}

EXTERNAL void tphAddFinalizer(void* obj) {
    tph_add_finalizer(obj);
}

EXTERNAL void* tphGetFinalizedObject() {
    return tph_get_finalized_object();
}

EXTERNAL void tphModifyCheck(void* ref) {
    tph_modify_check(ref);
}

EXTERNAL void tphHandleUserCollectionRequest(void* tls) {
    tph_handle_user_collection_request(tls);
}

EXTERNAL void tphHarnessBegin(void* tls) {
    tph_harness_begin(tls);
}

EXTERNAL void tphHarnessEnd() {
    tph_harness_end();
}

// #endif
