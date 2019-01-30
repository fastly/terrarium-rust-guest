//! Bindings to the hostcalls exposed for the demo.
//!
//! This API is probably not what you want to use directly. We
//! recommend instead using the `Request` and `Response` types
//! re-exported from the `http` crate, and the `RequestExt::send()`
//! method.
//!
//! Eventually this will be generated code, but `wasm-bindgen` isn't
//! currently what we want, since it assumes JavaScript will be on the
//! other side of the FFI.

pub mod raw;
pub mod types;

pub use crate::hostcalls::types::{
    GuestSlice, HostcallStatus, PendingRequestHandle, PollResult, RequestHandle, ResponseHandle,
};

use crate::guest_allocator::free;
use std::{ptr, slice};

impl RequestHandle {
    /// Create a new request.
    ///
    /// If request creation fails within the hostcall, this returns
    /// `None`. This is typically due to a malformed method or URL
    /// string.
    pub fn create(method: &str, url: &str) -> Option<RequestHandle> {
        let method_bytes = method.as_bytes();
        let url_bytes = url.as_bytes();
        let req = unsafe {
            raw::hostcall_req_create(
                method_bytes.as_ptr(),
                method_bytes.len(),
                url_bytes.as_ptr(),
                url_bytes.len(),
            )
        };
        let req = RequestHandle::from(req);
        if req.is_error() {
            None
        } else {
            Some(req)
        }
    }

    /// Synchronously send a request.
    ///
    /// Consumes the request, and returns a response. If the request fails, this returns
    /// `None`. Future versions of this API will offer more details about why a failure occurred,
    /// but the potential reasons are described in the [`reqwest`
    /// documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
    ///
    /// It is an error to call this method on `RequestHandle::INCOMING`.
    pub fn send(self) -> Option<ResponseHandle> {
        let resp = unsafe { raw::hostcall_req_send(self.into()) };
        let resp = ResponseHandle::from(resp);
        if resp.is_error() {
            None
        } else {
            Some(resp)
        }
    }

    /// Asynchronously send a request.
    ///
    /// Consumes the request, and returns a pending request. If request initialization fails, this
    /// returns `None`. Future versions of this API will offer more details about why a failure
    /// occurred, but the potential reasons are described in the [`reqwest`
    /// documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
    ///
    /// It is an error to call this method on `RequestHandle::INCOMING`.
    pub fn send_async(self) -> Option<PendingRequestHandle> {
        let resp = unsafe { raw::hostcall_req_send_async(self.into()) };
        let resp = PendingRequestHandle::from(resp);
        if resp.is_error() {
            None
        } else {
            Some(resp)
        }
    }

    /// Get the values associated in the request with a particular header name.
    ///
    /// It is an error to call this method on a request handle
    /// returned by `RequestHandle::create()`.
    pub fn get_header(&self, name: &str) -> Vec<String> {
        let name_bytes = name.as_bytes();
        let mut values_ptr: *mut GuestSlice<u8> = ptr::null_mut();
        let mut values_len: usize = 0;
        unsafe {
            raw::hostcall_req_get_header(
                &mut values_ptr,
                &mut values_len,
                self.into(),
                name_bytes.as_ptr(),
                name_bytes.len(),
            )
        };
        if values_len == 0 {
            return vec![];
        }
        assert!(!values_ptr.is_null());
        let values_slices = unsafe { slice::from_raw_parts_mut(values_ptr, values_len) };
        let mut values = vec![];
        for value_slice in values_slices {
            let value = String::from_utf8_lossy(unsafe { value_slice.to_slice() });
            values.push(value.to_string());
            free(value_slice.raw() as _);
        }
        free(values_ptr as _);
        values
    }

    /// Get the names of all headers in the request.
    ///
    /// It is an error to call this method on a request handle
    /// returned by `RequestHandle::create()`.
    pub fn get_headers(&self) -> Vec<String> {
        let mut headers_ptr: *mut GuestSlice<u8> = ptr::null_mut();
        let mut headers_len: usize = 0;
        unsafe { raw::hostcall_req_get_headers(&mut headers_ptr, &mut headers_len, self.into()) };
        if headers_len == 0 {
            return vec![];
        }
        assert!(!headers_ptr.is_null());
        let header_slices = unsafe { slice::from_raw_parts_mut(headers_ptr, headers_len) };
        let mut headers = vec![];
        for header_slice in header_slices {
            let header = String::from_utf8_lossy(unsafe { header_slice.to_slice() });
            headers.push(header.to_string());
            free(header_slice.raw() as _);
        }
        free(headers_ptr as _);
        headers
    }

    /// Get the HTTP method of the request.
    ///
    /// It is an error to call this method on a request handle
    /// returned by `RequestHandle::create()`.
    pub fn get_method(&self) -> String {
        let mut method_ptr: *mut u8 = ptr::null_mut();
        let mut method_len: usize = 0;
        unsafe { raw::hostcall_req_get_method(&mut method_ptr, &mut method_len, self.into()) };
        if method_len == 0 {
            return String::new();
        }
        assert!(!method_ptr.is_null());
        let method = unsafe { slice::from_raw_parts_mut(method_ptr, method_len) };
        let method = String::from_utf8_lossy(method).to_string();
        free(method_ptr as _);
        method
    }

    /// Get the body of the request as a vector of bytes.
    ///
    /// It is an error to call this method on a request handle
    /// returned by `RequestHandle::create()`.
    pub fn get_body(&self) -> Vec<u8> {
        let mut body_ptr: *mut u8 = ptr::null_mut();
        let mut body_len: usize = 0;
        unsafe { raw::hostcall_req_get_body(&mut body_ptr, &mut body_len, self.into()) };
        if body_len == 0 {
            return vec![];
        }
        assert!(!body_ptr.is_null());
        let body = unsafe { slice::from_raw_parts_mut(body_ptr, body_len) }.to_vec();
        free(body_ptr as _);
        body
    }

    /// Get the path of the request.
    ///
    /// It is an error to call this method on a request handle
    /// returned by `RequestHandle::create()`.
    pub fn get_path(&self) -> String {
        let mut path_ptr: *mut u8 = ptr::null_mut();
        let mut path_len: usize = 0;
        unsafe { raw::hostcall_req_get_path(&mut path_ptr, &mut path_len, self.into()) };
        if path_len == 0 {
            return String::new();
        }
        assert!(!path_ptr.is_null());
        let path = unsafe { slice::from_raw_parts_mut(path_ptr, path_len) };
        let path = String::from_utf8_lossy(path).to_string();
        free(path_ptr as _);
        path
    }

    /// Set a header to potentially-many values in the response.
    ///
    /// It is an error to call this method on `RequestHandle::INCOMING`.
    pub fn set_header(&mut self, name: &str, values: &[&str]) -> HostcallStatus {
        let name_bytes = name.as_bytes();
        let value_slices: Vec<GuestSlice<u8>> = values
            .iter()
            .map(|v| {
                let v_bytes = v.as_bytes();
                GuestSlice::new(v_bytes.as_ptr(), v_bytes.len())
            })
            .collect();
        unsafe {
            raw::hostcall_req_set_header(
                self.into(),
                name_bytes.as_ptr(),
                name_bytes.len(),
                value_slices.as_ptr(),
                value_slices.len(),
            )
        }
    }

    /// Set the body of the response.
    ///
    /// It is an error to call this method on `RequestHandle::INCOMING`.
    pub fn set_body(&mut self, body: &[u8]) -> HostcallStatus {
        unsafe { raw::hostcall_req_set_body(self.into(), body.as_ptr(), body.len()) }
    }
}

impl ResponseHandle {
    /// Get the names of all headers in the response.
    ///
    /// It is an error to call this method on `ResponseHandle::OUTGOING`.
    pub fn get_headers(&self) -> Vec<String> {
        let mut headers_ptr: *mut GuestSlice<u8> = ptr::null_mut();
        let mut headers_len: usize = 0;
        unsafe { raw::hostcall_resp_get_headers(&mut headers_ptr, &mut headers_len, self.into()) };
        if headers_len == 0 {
            return vec![];
        }
        assert!(!headers_ptr.is_null());
        let header_slices = unsafe { slice::from_raw_parts_mut(headers_ptr, headers_len) };
        let mut headers = vec![];
        for header_slice in header_slices {
            let header = String::from_utf8_lossy(unsafe { header_slice.to_slice() });
            headers.push(header.to_string());
            free(header_slice.raw() as _);
        }
        free(headers_ptr as _);
        headers
    }

    /// Get the values associated in the response with a particular header name.
    ///
    /// It is an error to call this method on `ResponseHandle::OUTGOING`.
    pub fn get_header(&self, name: &str) -> Vec<String> {
        let name_bytes = name.as_bytes();
        let mut values_ptr: *mut GuestSlice<u8> = ptr::null_mut();
        let mut values_len: usize = 0;
        unsafe {
            raw::hostcall_resp_get_header(
                &mut values_ptr,
                &mut values_len,
                self.into(),
                name_bytes.as_ptr(),
                name_bytes.len(),
            )
        };
        if values_len == 0 {
            return vec![];
        }
        assert!(!values_ptr.is_null());
        let values_slices = unsafe { slice::from_raw_parts_mut(values_ptr, values_len) };
        let mut values = vec![];
        for value_slice in values_slices {
            let value = String::from_utf8_lossy(unsafe { value_slice.to_slice() });
            values.push(value.to_string());
            free(value_slice.raw() as _);
        }
        free(values_ptr as _);
        values
    }

    /// Get the body of the response as a vector of bytes.
    ///
    /// It is an error to call this method on `ResponseHandle::OUTGOING`.
    pub fn get_body(&self) -> Vec<u8> {
        let mut body_ptr: *mut u8 = ptr::null_mut();
        let mut body_len: usize = 0;
        unsafe { raw::hostcall_resp_get_body(&mut body_ptr, &mut body_len, self.into()) };
        if body_len == 0 {
            return vec![];
        }
        assert!(!body_ptr.is_null());
        let body = unsafe { slice::from_raw_parts_mut(body_ptr, body_len) }.to_vec();
        free(body_ptr as _);
        body
    }

    /// Get the status code returned by the response.
    ///
    /// It is an error to call this method on `ResponseHandle::OUTGOING`.
    pub fn get_response_code(&self) -> u16 {
        let code = unsafe { raw::hostcall_resp_get_response_code(self.into()) };
        if code > ::std::u16::MAX as u32 {
            panic!(
                "response code returned from host was out of range: {}",
                code
            )
        } else {
            code as u16
        }
    }

    /// Set a header to potentially-many values in the response.
    ///
    /// Note: we're considering adding a variant of this that takes a
    /// `Vec<String>` rather than the somewhat awkward (but efficient)
    /// `&[&str]`.
    ///
    /// It is an error to call this method on a response handle
    /// returned by `RequestHandle::send()`.
    pub fn set_header(&mut self, name: &str, values: &[&str]) -> HostcallStatus {
        let name_bytes = name.as_bytes();
        let value_slices: Vec<GuestSlice<u8>> = values
            .iter()
            .map(|v| {
                let v_bytes = v.as_bytes();
                GuestSlice::new(v_bytes.as_ptr(), v_bytes.len())
            })
            .collect();
        unsafe {
            raw::hostcall_resp_set_header(
                self.into(),
                name_bytes.as_ptr(),
                name_bytes.len(),
                value_slices.as_ptr(),
                value_slices.len(),
            )
        }
    }

    /// Set the body of the response.
    ///
    /// It is an error to call this method on a response handle
    /// returned by `RequestHandle::send()`.
    pub fn set_body(&mut self, body: &[u8]) -> HostcallStatus {
        unsafe { raw::hostcall_resp_set_body(self.into(), body.as_ptr(), body.len()) }
    }

    /// Set the HTTP response code.
    ///
    /// It is an error to call this method on a response handle
    /// returned by `RequestHandle::send()`.
    pub fn set_response_code(&mut self, code: u16) -> HostcallStatus {
        unsafe { raw::hostcall_resp_set_response_code(self.into(), code) }
    }
}

impl PendingRequestHandle {
    /// Block until the request has completed.
    ///
    /// Consumes the pending request handle, and returns a response. If the request fails, this
    /// returns `None`. Future versions of this API will offer more details about why a failure
    /// occurred, but the potential reasons are described in the [`reqwest`
    /// documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
    pub fn wait(self) -> Option<ResponseHandle> {
        let resp = unsafe { raw::hostcall_pending_req_wait(self.into()) };
        let resp = ResponseHandle::from(resp);
        if resp.is_error() {
            None
        } else {
            Some(resp)
        }
    }

    /// Poll the status of the pending request without blocking.
    ///
    /// If the request has not completed, returns `PollResult::NotReady(pending_req)`, so that the
    /// pending request can be used again..
    ///
    /// If the request has completed, consumes the pending request handle, and returns a response in
    /// `PollResult::Response(resp)`.
    ///
    /// If the request fails, returns `PollResult::Error`. Future versions of this API will offer
    /// more details about why a failure occurred, but the potential reasons are described in the
    /// [`reqwest` documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
    pub fn poll(self) -> PollResult {
        let pr_raw = self.into();
        let resp = unsafe { raw::hostcall_pending_req_poll(pr_raw) };
        let resp = ResponseHandle::from(resp);
        if resp.is_error() {
            PollResult::Error
        } else if resp.is_not_ready() {
            PollResult::NotReady(PendingRequestHandle::from(pr_raw))
        } else {
            PollResult::Response(resp)
        }
    }
}

/// Select from a list of pending requests, blocking until one completes.
///
/// If a request succeeds, returns `Ok((pending_req, resp))` with the request that succeeded,
/// paired with its response.
///
/// If a request fails, returns `Err(pending_req)` with the request that failed. Future versions
/// of this API will offer more details about why a failure occurred, but the potential reasons
/// are described in the [`reqwest`
/// documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
///
/// **Note**: the `pending_req` value returned in both the success and error case is no longer
/// valid as an argument to `wait`, `poll`, or `select`; it is only returned in order to allow
/// identifying which request succeeded or errored.
///
/// All other pending requests passed to this function remain valid for subsequent calls.
pub fn select(
    prs: &[&PendingRequestHandle],
) -> Result<(PendingRequestHandle, ResponseHandle), PendingRequestHandle> {
    let prs = prs.iter().map(|&pr| i32::from(pr)).collect::<Vec<i32>>();
    let pr_out = Box::into_raw(Box::new(0));

    let resp = unsafe { raw::hostcall_pending_req_select(prs.as_ptr(), prs.len(), pr_out) };
    let resp = ResponseHandle::from(resp);
    let pr_out = unsafe { PendingRequestHandle::from(*Box::from_raw(pr_out)) };
    if resp.is_error() {
        Err(pr_out)
    } else if resp.is_not_ready() {
        panic!("`select` should never return ResponseHandle::NOT_READY")
    } else {
        Ok((pr_out, resp))
    }
}

pub fn kvstore_insert(key: &str, value: &[u8]) -> bool {
    let key_bytes = key.as_bytes();
    unsafe {
        raw::hostcall_kvstore_insert(
            key_bytes.as_ptr(),
            key_bytes.len(),
            value.as_ptr(),
            value.len(),
        )
    }
}

pub fn kvstore_upsert(key: &str, value: &[u8]) -> bool {
    let key_bytes = key.as_bytes();
    unsafe {
        raw::hostcall_kvstore_upsert(
            key_bytes.as_ptr(),
            key_bytes.len(),
            value.as_ptr(),
            value.len(),
        )
    }
}

pub fn kvstore_append(key: &str, value: &[u8]) -> bool {
    let key_bytes = key.as_bytes();
    unsafe {
        raw::hostcall_kvstore_append(
            key_bytes.as_ptr(),
            key_bytes.len(),
            value.as_ptr(),
            value.len(),
        )
    }
}

pub fn kvstore_get(key: &str) -> Option<Vec<u8>> {
    let key_bytes = key.as_bytes();
    let mut value_ptr: *mut u8 = ptr::null_mut();
    let mut value_len: usize = 0;
    let found = unsafe {
        raw::hostcall_kvstore_get(
            &mut value_ptr,
            &mut value_len,
            key_bytes.as_ptr(),
            key_bytes.len(),
        )
    };
    match found {
        false => None,
        true => {
            // Host should not send a null pointer if found is true, but guard on it anyway
            // because it is UB in slice::from_raw_parts_mut. Thanks @pepyakin
            assert!(!value_ptr.is_null());
            let value = unsafe { slice::from_raw_parts_mut(value_ptr, value_len) }.to_vec();
            free(value_ptr as _);
            Some(value)
        }
    }
}

pub fn kvstore_remove(key: &str) -> bool {
    let key_bytes = key.as_bytes();
    unsafe { raw::hostcall_kvstore_remove(key_bytes.as_ptr(), key_bytes.len()) }
}

pub fn debug(msg: &str) {
    let msg_bytes = msg.as_bytes();
    unsafe { raw::hostcall_debug(msg_bytes.as_ptr(), msg_bytes.len()) }
}

/// Pass the message from the panic to `hostcall_panic_hook`, so that
/// the runtime can determine how to handle it.
pub(crate) fn panic_hook(msg: &str) {
    let msg_bytes = msg.as_bytes();
    unsafe { raw::hostcall_panic_hook(msg_bytes.as_ptr(), msg_bytes.len()) }
}
