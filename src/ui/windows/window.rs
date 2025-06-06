use windows::{core::*, Win32::{Foundation::*, UI::WindowsAndMessaging::*}};

struct Window {
    target: HWND
}
impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyWindow(self.target);
        }
    }
}
impl Window {
    
}
