use hostcalls::types::{GuestSlice, HostcallStatus};
use std::os::raw::c_void;

extern "C" {
    pub fn hostcall_req_create(
        method_ptr: *const u8,
        method_len: usize,
        url_ptr: *const u8,
        url_len: usize,
    ) -> i32;

    pub fn hostcall_req_send(req: i32) -> i32;

    pub fn hostcall_req_send_async(req: i32) -> i32;

    pub fn hostcall_pending_req_wait(pr: i32) -> i32;

    pub fn hostcall_pending_req_poll(pr: i32) -> i32;

    pub fn hostcall_pending_req_select(
        prs_ptr: *const i32,
        prs_len: usize,
        pr_out: *mut i32,
    ) -> i32;

    pub fn hostcall_req_get_header(
        values_ptr_p: *mut *mut GuestSlice<u8>,
        values_len_p: *mut usize,
        req: i32,
        name_ptr: *const u8,
        name_len: usize,
    );

    pub fn hostcall_req_get_headers(
        headers_ptr_p: *mut *mut GuestSlice<u8>,
        headers_len_p: *mut usize,
        req: i32,
    );

    pub fn hostcall_req_get_method(method_ptr_p: *mut *mut u8, method_len_p: *mut usize, req: i32);

    pub fn hostcall_req_get_body(body_ptr_p: *mut *mut u8, body_len_p: *mut usize, req: i32);

    pub fn hostcall_req_get_path(path_ptr_p: *mut *mut u8, path_len_p: *mut usize, req: i32);

    pub fn hostcall_req_set_header(
        req: i32,
        name_ptr: *const u8,
        name_len: usize,
        values_slice_ptr: *const GuestSlice<u8>,
        values_slice_len: usize,
    ) -> HostcallStatus;

    pub fn hostcall_req_set_body(req: i32, body_ptr: *const u8, body_len: usize) -> HostcallStatus;

    pub fn hostcall_resp_get_headers(
        headers_ptr_p: *mut *mut GuestSlice<u8>,
        headers_len_p: *mut usize,
        resp: i32,
    );

    pub fn hostcall_resp_get_header(
        values_ptr_p: *mut *mut GuestSlice<u8>,
        values_len_p: *mut usize,
        resp: i32,
        name_ptr: *const u8,
        name_len: usize,
    );

    pub fn hostcall_resp_get_body(body_ptr_p: *mut *mut u8, body_len_p: *mut usize, resp: i32);

    pub fn hostcall_resp_get_response_code(resp: i32) -> u32;

    pub fn hostcall_resp_set_header(
        resp: i32,
        name_ptr: *const u8,
        name_len: usize,
        values_ptr_p: *const GuestSlice<u8>,
        values_len_p: usize,
    ) -> HostcallStatus;

    pub fn hostcall_resp_set_body(
        resp: i32,
        body_ptr: *const u8,
        body_len: usize,
    ) -> HostcallStatus;

    pub fn hostcall_resp_set_response_code(resp: i32, code: u16) -> HostcallStatus;

    pub fn hostcall_kvstore_insert(
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    ) -> bool;

    pub fn hostcall_kvstore_upsert(
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    ) -> bool;

    pub fn hostcall_kvstore_append(
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    ) -> bool;

    pub fn hostcall_kvstore_get(
        value_ptr_p: *mut *mut u8,
        value_len_p: *mut usize,
        key_ptr: *const u8,
        key_len: usize,
    ) -> bool;

    pub fn hostcall_kvstore_remove(key_ptr: *const u8, key_len: usize) -> bool;

    pub fn hostcall_panic_hook(msg_ptr: *const u8, msg_len: usize);

    pub fn hostcall_init_mm(
        malloc_impl: extern "C" fn(size: usize) -> *mut c_void,
        free_impl: extern "C" fn(ptr: *mut c_void),
    );

    pub fn hostcall_rng_next_u64() -> u64;

    pub fn hostcall_time_now(subsec_nanos_p: *mut u32) -> u64;

    pub fn hostcall_dns_query_raw(
        response_ptr_p: *mut *mut u8,
        response_len_p: *mut usize,
        query_ptr: *const u8,
        query_len: usize,
    ) -> bool;

    pub fn hostcall_dns_query_ip(
        responses_ptr_p: *mut *mut GuestSlice<u8>,
        responses_len_p: *mut usize,
        name_ptr: *const u8,
        name_len: usize,
        ipv6: bool,
    ) -> bool;

    pub fn hostcall_debug(msg_ptr: *const u8, msg_len: usize);
}
