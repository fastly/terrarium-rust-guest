use std::mem;
use std::slice;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum HostcallStatus {
    Ok = 0,
    Invalid = 1,
}

impl HostcallStatus {
    #[allow(dead_code)]
    pub fn try_from_u8(v: u8) -> Option<HostcallStatus> {
        use self::HostcallStatus::*;
        match v {
            0 => Some(Ok),
            1 => Some(Invalid),
            _ => None,
        }
    }
}

#[repr(C)]
pub struct GuestSlice<T> {
    ptr: *const T,
    len: usize,
}

impl<T> GuestSlice<T> {
    pub fn new(ptr: *const T, len: usize) -> Self {
        GuestSlice { ptr, len }
    }

    pub unsafe fn to_slice<'a>(&self) -> &'a [T] {
        if self.len % mem::size_of::<T>() != 0 {
            panic!("GuestSlice didn't contain a multiple of element size");
        }

        let n_elts = self.len / mem::size_of::<T>();
        slice::from_raw_parts(self.ptr, n_elts)
    }

    pub fn raw(&self) -> *const T {
        self.ptr
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct RequestHandle(i32);

impl From<i32> for RequestHandle {
    fn from(i: i32) -> RequestHandle {
        RequestHandle(i)
    }
}

impl From<RequestHandle> for i32 {
    fn from(req: RequestHandle) -> i32 {
        req.0
    }
}

impl<'a> From<&'a RequestHandle> for i32 {
    fn from(req: &RequestHandle) -> i32 {
        req.0
    }
}

impl<'a> From<&'a mut RequestHandle> for i32 {
    fn from(req: &mut RequestHandle) -> i32 {
        req.0
    }
}

impl RequestHandle {
    /// The request corresponding to the request incoming to the handler
    pub const INCOMING: RequestHandle = RequestHandle(0);

    /// Sentinel value to represent errors
    pub const ERROR: RequestHandle = RequestHandle(-1);

    pub fn is_error(&self) -> bool {
        *self == RequestHandle::ERROR
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ResponseHandle(i32);

impl From<i32> for ResponseHandle {
    fn from(i: i32) -> ResponseHandle {
        ResponseHandle(i)
    }
}

impl From<ResponseHandle> for i32 {
    fn from(resp: ResponseHandle) -> i32 {
        resp.0
    }
}

impl<'a> From<&'a ResponseHandle> for i32 {
    fn from(resp: &ResponseHandle) -> i32 {
        resp.0
    }
}

impl<'a> From<&'a mut ResponseHandle> for i32 {
    fn from(resp: &mut ResponseHandle) -> i32 {
        resp.0
    }
}

impl ResponseHandle {
    /// The response corresponding to the outgoing response from the handler
    pub const OUTGOING: ResponseHandle = ResponseHandle(0);

    /// Sentinel value to represent errors
    pub const ERROR: ResponseHandle = ResponseHandle(-1);

    pub fn is_error(&self) -> bool {
        *self == ResponseHandle::ERROR
    }

    /// Returned when a pending request is not complete, but we don't want to block
    pub const NOT_READY: ResponseHandle = ResponseHandle(-2);

    pub fn is_not_ready(&self) -> bool {
        *self == ResponseHandle::NOT_READY
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct PendingRequestHandle(i32);

impl From<i32> for PendingRequestHandle {
    fn from(i: i32) -> PendingRequestHandle {
        PendingRequestHandle(i)
    }
}

impl From<PendingRequestHandle> for i32 {
    fn from(resp: PendingRequestHandle) -> i32 {
        resp.0
    }
}

impl<'a> From<&'a PendingRequestHandle> for i32 {
    fn from(resp: &PendingRequestHandle) -> i32 {
        resp.0
    }
}

impl<'a> From<&'a mut PendingRequestHandle> for i32 {
    fn from(resp: &mut PendingRequestHandle) -> i32 {
        resp.0
    }
}

impl PendingRequestHandle {
    /// Sentinel value to represent errors
    pub const ERROR: PendingRequestHandle = PendingRequestHandle(-1);

    pub fn is_error(&self) -> bool {
        *self == PendingRequestHandle::ERROR
    }
}

#[derive(Debug, PartialEq)]
pub enum PollResult {
    NotReady(PendingRequestHandle),
    Response(ResponseHandle),
    Error,
}
