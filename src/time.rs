use coarsetime::Duration;
use hostcalls::raw::hostcall_time_now;

pub struct Time {}

impl Time {
    pub fn since_epoch() -> Duration {
        let mut subsec_nanos: u32 = 0;
        let secs = unsafe { hostcall_time_now(&mut subsec_nanos) };
        Duration::new(secs, subsec_nanos)
    }
}
