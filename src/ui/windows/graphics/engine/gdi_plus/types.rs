
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

use windows::Win32::Graphics::GdiPlus::{GdipCreateFromHDC, GdipCreateFromHWND, GdipDeleteBrush, GdipDeleteGraphics, GpGraphics, Status, GpSolidFill, GpBrush};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HDC;

pub struct Graphics {
    data: GpGraphics
}
impl Deref for Graphics {
    type Target = GpGraphics;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for Graphics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe {
            GdipDeleteGraphics(&mut self.data);
        }
    }
}
impl Graphics {
    pub fn new_hdc(dc: HDC) -> Result<Self, Status> {
        let mut m_data: Box<MaybeUninit<GpGraphics>> = Box::new_uninit();
        let data: GpGraphics;
        unsafe {
            let result = GdipCreateFromHDC(dc, &mut m_data.as_mut_ptr());
            if result.0 != 0 {
                return Err(result);
            }

            data = *m_data.assume_init();
        }

        Ok(
            Self {
                data
            }
        )
    }
    pub fn new_hwnd(wnd: HWND) -> Result<Self, Status> {
        let mut m_data: Box<MaybeUninit<GpGraphics>> = Box::new_uninit();
        let data: GpGraphics;
        unsafe {
            let result = GdipCreateFromHWND(wnd, &mut m_data.as_mut_ptr());
            if result.0 != 0 {
                return Err(result);
            }

            data = *m_data.assume_init()
        }

        Ok(
            Self {
                data
            }
        )
    }
}

pub struct SolidBrush {
    data: GpSolidFill
}
impl Drop for SolidBrush {
    fn drop(&mut self) {
        unsafe {
            let mut conv: *mut GpBrush = &mut self.data as &mut GpBrush;
            GdipDeleteBrush(&mut self.data);
        }
    }
}

pub struct TextureBrush {

}

pub struct LineGradient {

}

pub struct Pen {
    
}

pub struct Font {

}

pub struct FontFamily {

}

pub struct StringFormat {

}

pub struct Bitmap {

}

pub struct Image {

}

pub struct Region {

}

pub struct Matrix {

}