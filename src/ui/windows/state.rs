use windows::{core::*, Win32::{Foundation::*, UI::WindowsAndMessaging::*}};

struct WindowState {
    handle: HWND,
    id: u64
}

