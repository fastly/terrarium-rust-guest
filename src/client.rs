use failure::Fail;
use http::{self, Request, Response};

use crate::hostcalls;
use crate::hostcalls::types::{
    self as hostcall_types, HostcallStatus, PendingRequestHandle, RequestHandle, ResponseHandle,
};

#[derive(Debug, Fail)]
pub enum SendError {
    /// An error that arises within the host.
    ///
    /// Future versions of this API will provide more detail on the
    /// type of failure, but this currently includes connection errors
    /// and malformed message payloads.
    #[fail(display = "Hostcall send error")]
    Hostcall,
    /// Arises when a header cannot be converted to a string.
    #[fail(display = "Header to str send error: {}", _0)]
    HeaderToStr(http::header::ToStrError),
    /// Can arise during creation of the response within the guest.
    #[fail(display = "Http send error: {}", _0)]
    Http(http::Error),
}

#[derive(Debug, PartialEq)]
pub struct PendingRequest(PendingRequestHandle);

pub enum PollResult {
    NotReady(PendingRequest),
    Response(Response<Vec<u8>>),
}

pub trait RequestExt {
    type R;
    type Pending;

    /// Synchronously send a request, and return a response.
    ///
    /// This will consume the request and block until returning either a response, or a
    /// `SendError`. Most of the detailed error conditions relate to parsing of characters in
    /// headers, but future versions of this API will provide more detail on the types of failures
    /// that occur behind hostcalls.
    ///
    /// Note: the `version` field on the request is currently ignored.
    fn send(self) -> Result<Self::R, SendError>;

    /// Asynchronously send a request.
    ///
    /// This will consume the request and immediately return either a pending request, or a
    /// `SendError`. Most of the detailed error conditions relate to parsing of characters in
    /// headers, but future versions of this API will provide more detail on the types of failures
    /// that occur behind hostcalls.
    fn send_async(self) -> Result<Self::Pending, SendError>;
}

impl RequestExt for Request<Vec<u8>> {
    type R = Response<Vec<u8>>;
    type Pending = PendingRequest;

    fn send(self) -> Result<Response<Vec<u8>>, SendError> {
        let req = prepare_req(self)?;

        let resp_handle = req.send().ok_or(SendError::Hostcall)?;

        build_response(resp_handle)
    }

    fn send_async(self) -> Result<Self::Pending, SendError> {
        let req = prepare_req(self)?;

        req.send_async()
            .map(PendingRequest)
            .ok_or(SendError::Hostcall)
    }
}

impl PendingRequest {
    /// Block until the request has completed.
    ///
    /// Consumes the pending request handle, and returns a response. If the request fails, this
    /// returns a `SendError`. Future versions of this API will offer more details about why a
    /// failure occurred, but the potential reasons are described in the [`reqwest`
    /// documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
    pub fn wait(self) -> Result<Response<Vec<u8>>, SendError> {
        let resp_handle = self.0.wait().ok_or(SendError::Hostcall)?;

        build_response(resp_handle)
    }

    /// Poll the status of the pending request without blocking.
    ///
    /// If the request has not completed, returns `PollResult::NotReady(pending_req)`, so that the
    /// pending request can be used again..
    ///
    /// If the request has completed, consumes the pending request handle, and returns a response in
    /// `PollResult::Response(resp)`.
    ///
    /// If the request fails, returns `Err(SendError::Hostcall)`. Future versions of this API will
    /// offer more details about why a failure occurred, but the potential reasons are described in
    /// the [`reqwest` documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
    pub fn poll(self) -> Result<PollResult, SendError> {
        match self.0.poll() {
            hostcall_types::PollResult::Response(resp_handle) => {
                build_response(resp_handle).map(PollResult::Response)
            }
            hostcall_types::PollResult::NotReady(pr_handle) => {
                Ok(PollResult::NotReady(PendingRequest(pr_handle)))
            }
            hostcall_types::PollResult::Error => Err(SendError::Hostcall),
        }
    }
}

/// Select from a list of pending requests, blocking until one completes.
///
/// If a request succeeds, returns `Ok((pending_req, resp))` with the request that succeeded, paired
/// with its response.
///
/// If a request fails, returns `Err(pending_req)` with the request that failed. Future versions of
/// this API will offer more details about why a failure occurred, but the potential reasons are
/// described in the [`reqwest`
/// documentation](https://docs.rs/reqwest/0.9.3/reqwest/struct.Error.html).
///
/// **Note**: the `pending_req` value returned in both the success and error case is no longer valid
/// as an argument to `wait`, `poll`, or `select`; it is only returned in order to allow
/// identifying which request succeeded or errored.
///
/// All other pending requests passed to this function remain valid for subsequent calls.
pub fn select(
    prs: &[&PendingRequest],
) -> Result<(PendingRequest, Response<Vec<u8>>), PendingRequest> {
    let pr_handles = prs
        .iter()
        .map(|pr| &pr.0)
        .collect::<Vec<&PendingRequestHandle>>();
    match hostcalls::select(&pr_handles) {
        Ok((pr, resp)) => {
            let pr = PendingRequest(pr);
            if let Ok(resp) = build_response(resp) {
                Ok((pr, resp))
            } else {
                // request succeeded, but something went wrong
                // building the response. For now, we just treat it as
                // though the whole thing failed
                Err(pr)
            }
        }
        Err(pr) => Err(PendingRequest(pr)),
    }
}

fn prepare_req(req: Request<Vec<u8>>) -> Result<RequestHandle, SendError> {
    let (parts, body) = req.into_parts();

    let mut req = RequestHandle::create(parts.method.as_str(), &parts.uri.to_string())
        .ok_or(SendError::Hostcall)?;

    for key in parts.headers.keys() {
        let mut vs = vec![];
        for v in parts.headers.get_all(key) {
            let v_str = v.to_str().map_err(SendError::HeaderToStr)?;
            vs.push(v_str);
        }
        if req.set_header(key.as_str(), &vs) == HostcallStatus::Invalid {
            return Err(SendError::Hostcall);
        }
    }

    if req.set_body(&body) == HostcallStatus::Invalid {
        return Err(SendError::Hostcall);
    }

    Ok(req)
}

fn build_response(resp_handle: ResponseHandle) -> Result<Response<Vec<u8>>, SendError> {
    let mut resp = Response::builder();

    for key in resp_handle.get_headers() {
        for v in resp_handle.get_header(&key) {
            resp.header(key.as_str(), v);
        }
    }

    resp.status(resp_handle.get_response_code());

    resp.body(resp_handle.get_body()).map_err(SendError::Http)
}
