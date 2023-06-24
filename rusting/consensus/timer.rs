use std::thread;
use std::time::Duration;

pub fn wait(val: u64)
{
    thread::sleep(Duration::from_millis(val));
}