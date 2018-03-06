extern crate winapi;

use std::io::Error;
use winapi::um::winuser::{HWND_BROADCAST, SC_MONITORPOWER, SendMessageA, WM_SYSCOMMAND};

fn main() {
    turn_off_monitor().unwrap();
}

fn turn_off_monitor() -> Result<isize, Error> {
    let ret = unsafe {
        SendMessageA(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, 0x0002) // POWER_OFF = 0x0002
    };

    if ret == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret)
    }
}