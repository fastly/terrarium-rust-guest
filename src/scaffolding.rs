//! Scaffolding for a guest application.

use http::{Request, Response};

pub use hostcalls::types::{RequestHandle, ResponseHandle};
use kvstore::KVStore;

/// Macro to set up the scaffolding
///
/// This macro takes care of setting up the scaffolding for the guest
/// code, including allocation, error handling, and conversion to and
/// from the `http` crate's types. The template for user code should
/// look something like this:
///
/// ```text
/// #[macro_use]
/// extern crate http_guest;
///
/// use http_guest::{Request, Response};
///
/// pub fn user_entrypoint(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
///     if req.method().as_str() == "POST" {
///         Response::builder().status(200).body("Hello!".as_bytes().to_owned())
///     } else {
///         Response::builder().status(405).body(vec![])
///     }
/// }
///
/// guest_app!(user_entrypoint);
/// ```
#[macro_export]
macro_rules! guest_app {
    ($user_entrypoint:ident) => {
        // // currently broken; see the string_pushes test
        // extern crate wee_alloc;

        // #[global_allocator]
        // static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

        #[no_mangle]
        pub extern "C" fn run() {
            http_guest::panic_set_once();
            http_guest::init_mm_default();
            http_guest::raw_entrypoint($user_entrypoint);
        }
    };
}

/// Variation on `guest_app` for applications that use the
/// cross-request key-value store.
///
/// ```text
/// #[macro_use]
/// extern crate http_guest;
///
/// use http_guest::{KVStore, Request, Response};
///
/// pub fn user_entrypoint(kvs: &mut KVStore, req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
///     if req.method().as_str() == "POST" {
///         kvs.insert("saw_a_post", b"yes!");
///         Response::builder().status(200).body("Hello!".as_bytes().to_owned())
///     } else {
///         Response::builder().status(405).body(vec![])
///     }
/// }
///
/// guest_app_kvs!(user_entrypoint);
/// ```
#[macro_export]
macro_rules! guest_app_kvs {
    ($user_entrypoint:ident) => {
        // // currently broken; see the string_pushes test
        // extern crate wee_alloc;

        // #[global_allocator]
        // static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

        #[no_mangle]
        pub extern "C" fn run() {
            http_guest::panic_set_once();
            http_guest::init_mm_default();
            http_guest::raw_entrypoint_kvs($user_entrypoint);
        }
    };
}

/// The entrypoint that uses hostcalls to create and consume the
/// `Request` and `Response` for the user entrypoint.
///
/// This is meant to be used by the `guest_app` scaffolding macro, and
/// should not be called directly by user code. We hide the doc so
/// that users aren't encouraged to mess with it, but it needs to be
/// exported so the macro can work.
#[doc(hidden)]
pub fn raw_entrypoint<F>(user_entrypoint: F)
where
    F: Fn(&Request<Vec<u8>>) -> Response<Vec<u8>>,
{
    build_resp(user_entrypoint(&build_req()));
}

pub fn raw_entrypoint_kvs<F>(user_entrypoint: F)
where
    F: Fn(&mut KVStore, &Request<Vec<u8>>) -> Response<Vec<u8>>,
{
    build_resp(user_entrypoint(&mut KVStore::global(), &build_req()));
}

/// Build up the `Request` from the hostcall interface
fn build_req() -> Request<Vec<u8>> {
    let inc = RequestHandle::INCOMING;
    let mut builder = Request::builder();
    for name in inc.get_headers() {
        for value in inc.get_header(&name) {
            builder.header(name.as_str(), value);
        }
    }
    builder
        .method(inc.get_method().as_str())
        .uri(inc.get_path())
        .body(inc.get_body())
        .unwrap()
}

/// Output the `Response` via hostcalls
fn build_resp(resp: Response<Vec<u8>>) {
    let mut out = ResponseHandle::OUTGOING;
    let headers = resp.headers();
    for name in headers.keys() {
        let values: Vec<&str> = headers
            .get_all(name)
            .iter()
            .map(|v| v.to_str().unwrap())
            .collect();
        out.set_header(name.as_str(), &values);
    }
    out.set_body(resp.body());
    out.set_response_code(resp.status().as_u16());
}
