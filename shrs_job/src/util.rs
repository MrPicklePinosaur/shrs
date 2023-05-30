use std::os::fd::{AsRawFd, RawFd};

pub fn get_terminal() -> RawFd {
    std::io::stdin().as_raw_fd()
}
