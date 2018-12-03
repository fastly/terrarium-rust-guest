//! API for guest applications in the `isolation-demo` environment.

extern crate coarsetime;
extern crate failure;
extern crate http;
extern crate rand_core;

mod client;
mod dns;
mod guest_allocator;
pub mod hostcalls;
pub mod kvstore;
mod panic;
pub mod rand;
pub mod time;
#[macro_use]
mod scaffolding;

pub use client::{select, PendingRequest, PollResult, RequestExt, SendError};
pub use dns::DNS;
pub use http::{header, Error, HeaderMap, Method, Request, Response, StatusCode, Uri};
pub use kvstore::KVStore;
pub use time::Time;

// export these for the scaffolding macro
pub use guest_allocator::init_mm_default;
pub use panic::panic_set_once;
pub use scaffolding::{raw_entrypoint, raw_entrypoint_kvs};
