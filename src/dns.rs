use crate::guest_allocator::free;
use crate::hostcalls::{
    raw::{hostcall_dns_query_ip, hostcall_dns_query_raw},
    types::GuestSlice,
};
use failure::Fail;
use std::{
    fmt::{self, Display, Formatter},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    ptr, slice,
};

pub struct DNS;

#[derive(Debug, Fail)]
pub struct DNSError(&'static str);

impl From<&'static str> for DNSError {
    fn from(description: &'static str) -> Self {
        DNSError(description)
    }
}

impl Display for DNSError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DNS {
    pub fn query_raw(query: &[u8]) -> Result<Vec<u8>, DNSError> {
        let mut response_ptr = ptr::null_mut();
        let mut response_len: usize = 0;
        unsafe {
            hostcall_dns_query_raw(
                &mut response_ptr,
                &mut response_len,
                query.as_ptr(),
                query.len(),
            )
        };
        if response_ptr.is_null() {
            Err(DNSError::from("Unable to resolve a raw query"))?
        }
        let response = unsafe { slice::from_raw_parts_mut(response_ptr, response_len) }.to_vec();
        free(response_ptr as _);
        Ok(response)
    }

    pub fn query_ip(name: &str, ipv6: bool) -> Result<Vec<IpAddr>, DNSError> {
        let name_bytes = name.as_bytes();
        let mut responses_ptr: *mut GuestSlice<u8> = ptr::null_mut();
        let mut responses_len: usize = 0;
        unsafe {
            hostcall_dns_query_ip(
                &mut responses_ptr,
                &mut responses_len,
                name_bytes.as_ptr(),
                name_bytes.len(),
                ipv6,
            )
        };
        if responses_ptr.is_null() {
            Err(DNSError::from("Unable to resolve an IP query"))?
        }
        let responses_slices = unsafe { slice::from_raw_parts_mut(responses_ptr, responses_len) };
        let mut responses = vec![];
        for response_slice in responses_slices {
            let response_bytes = unsafe { response_slice.to_slice() };
            let response = match response_bytes.len() {
                4 => {
                    let mut ip = [0u8; 4];
                    ip.copy_from_slice(response_bytes);
                    IpAddr::V4(Ipv4Addr::from(ip))
                }
                16 => {
                    let mut ip = [0u8; 16];
                    ip.copy_from_slice(response_bytes);
                    IpAddr::V6(Ipv6Addr::from(ip))
                }
                _ => unreachable!(),
            };
            responses.push(response);
            free(response_slice.raw() as _);
        }
        free(responses_ptr as _);
        Ok(responses)
    }
}
