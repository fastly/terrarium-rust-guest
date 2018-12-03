use rand_core::{impls, Error, RngCore};

use hostcalls::raw::hostcall_rng_next_u64;

/// Random number source for guest programs.
///
/// This is backed by a random number generator in the host, and thus has the same cryptographic
/// properties as [`ThreadRng`](https://docs.rs/rand/0.5.5/rand/rngs/struct.ThreadRng.html);
#[derive(Clone, Debug)]
pub struct GuestRng {
    _private: (),
}

/// Get the RNG for this guest.
pub fn guest_rng() -> GuestRng {
    GuestRng { _private: () }
}

impl RngCore for GuestRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        unsafe { hostcall_rng_next_u64() }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
