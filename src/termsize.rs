use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::mem::zeroed;

/// Runs the ioctl command. Returns (0, 0) if the output is not to a terminal, or
/// there is an error. (0, 0) is an invalid size to have anyway, which is why
/// it can be used as a nil value.
pub fn get_dimensions_out() -> winsize {
    unsafe {
        let mut window: winsize = zeroed();
        let result = ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut window);

        if result != -1 {
            return window;
        }
        zeroed()
    }
}
