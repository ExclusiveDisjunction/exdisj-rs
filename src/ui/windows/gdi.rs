use std::rc::Rc;
use std::{collections::HashMap, fmt::Display};
use std::mem::MaybeUninit;

use windows::Win32::Foundation::COLORREF;
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, DeleteObject, RoundRect, SetBkMode, HGDIOBJ, TRANSPARENT};
use windows::Win32::{Foundation::{HWND, RECT}, Graphics::Gdi::{BeginPaint, EndPaint, FillRect, HBRUSH, HDC, HFONT, PAINTSTRUCT}, UI::WindowsAndMessaging::GetWindowRect};

use super::graphics::{ResourceKey, GraphicsFrameContext, GraphicsViewContext, GraphicsWindowContext, GraphicsAppContext, GraphicsEngine, GraphicsResourceBook, FontStyle};
use crate::ui::tool::utf_str::Utf16String;

#[derive(Debug)]
pub enum GdiError {
    Windows(windows::core::Error),
    MissingResource(ResourceKey)
}
impl Display for GdiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Windows(win) => write!(f, "windows error: '{win}'"),
            Self::MissingResource(rc) => write!(f, "missing resource, key: '{rc}'")
        }
    }
}
impl std::error::Error for GdiError { }

pub struct GdiFrameContext {
    rc: Rc<GdiResourceBook>,
    ps: PAINTSTRUCT,
    hdc: HDC,
    rect: RECT,
    wnd: HWND
}
impl GraphicsFrameContext for GdiFrameContext {
    type Error = GdiError;
    type Unit = i32;
    type Rect = RECT;
    
    fn draw_bk(&mut self, color: ResourceKey) -> Result<(), Self::Error> {
        self.draw_rect(self.rect.clone(), None, color)
    }
    fn draw_rect(&mut self, size: Self::Rect, radius: Option<Self::Unit>, color: ResourceKey) -> Result<(), Self::Error> {
        let brush = match self.rc.get_color(&color) {
            Some(b) => b.clone(),
            None => return Err( GdiError::MissingResource(color) )
        };

        unsafe {
            if let Some(r) = radius {
                todo!()
                //RoundRect(hdc, left, top, right, bottom, width, height)
            }
            else {
                FillRect(self.hdc, &size, brush);
            }
        }

        Ok( () ) 
    }
    fn draw_frame(&mut self, size: Self::Rect, radius: Option<Self::Unit>, thickness: Self::Unit, color: ResourceKey, bk: ResourceKey) -> Result<(), Self::Error> {
        todo!()
    }
    fn draw_circle(&mut self, radius: Self::Unit, x: Self::Unit, y: Self::Unit, color: ResourceKey) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn write_text(&mut self, text: &Utf16String, x: Self::Unit, y: Self::Unit, style: FontStyle) -> Result<(), Self::Error> {
        todo!()
    }
}
impl Drop for GdiFrameContext {
    fn drop(&mut self) {
        unsafe {
            let _ = EndPaint(self.wnd, &self.ps);
        }
    }
}

pub struct GdiViewContext {
    rc: Rc<GdiResourceBook>,
    target: HWND
}
impl GraphicsViewContext for GdiViewContext {
    type Error = GdiError;
    type Frame = GdiFrameContext;

    fn make_frame(&mut self) -> Result<Self::Frame, Self::Error> {
        unsafe {
            let mut ps_box: Box<MaybeUninit<PAINTSTRUCT>> = Box::new_uninit();
            let mut rect_box: Box<MaybeUninit<RECT>> = Box::new_uninit();

            let dc = BeginPaint(self.target, ps_box.as_mut_ptr());
            if let Err(e) = GetWindowRect(self.target, rect_box.as_mut_ptr()) {
                return Err( GdiError::Windows(e) )
            }

            let ps = *ps_box.assume_init();
            let rect = *rect_box.assume_init();

            SetBkMode(dc, TRANSPARENT);

            Ok(
                Self::Frame {
                    rc: Rc::clone(&self.rc),
                    ps,
                    hdc: dc,
                    rect,
                    wnd: self.target
                }
            )
        }
    }
}

pub struct GdiWindowContext {
    rc: Rc<GdiResourceBook>
}
impl GraphicsWindowContext for GdiWindowContext {
    type Error = GdiError;
    type ViewContext = GdiViewContext;

    fn make_view_context(&mut self, over: HWND) -> Result<Self::ViewContext, Self::Error> {
        Ok(
            GdiViewContext {
                rc: Rc::clone(&self.rc),
                target: over
            }
        )
    }
}

pub struct GdiResourceBook {
    colors: HashMap<ResourceKey, HBRUSH>,
    fonts: HashMap<ResourceKey, HFONT>
}
impl GraphicsResourceBook for GdiResourceBook {
    type Color = HBRUSH;
    type Font = HFONT;

    fn get_color(&self, key: &ResourceKey) -> Option<&Self::Color> {
        self.colors.get(&key)
    }
    fn get_font(&self, key: &ResourceKey) -> Option<&Self::Font> {
        self.fonts.get(&key)
    }
}
impl Drop for GdiResourceBook {
    fn drop(&mut self) {
        let colors_iter = self.colors.values()
            .cloned()
            .map(|x| { let x: HGDIOBJ = x.into(); x } );
        let fonts_iter = self.fonts.values()
            .cloned()
            .map(|x| { let x: HGDIOBJ = x.into(); x } );

        unsafe {
            colors_iter.chain(fonts_iter)
                .for_each(|x| {
                    let _ = DeleteObject(x);
                });
        }
    }
}
impl Default for GdiResourceBook {
    fn default() -> Self {
        Self {
            colors: HashMap::new(),
            fonts: HashMap::new()
        }
    }
}
impl GdiResourceBook {
    pub fn register_color(&mut self, key: ResourceKey, value: COLORREF) {
        let brush: HBRUSH;
        unsafe {
            brush = CreateSolidBrush(value);
        }

        self.register_brush(key, brush);
    }
    pub fn register_brush(&mut self, key: ResourceKey, brush: HBRUSH) {
        if let Some(old) = self.colors.insert(key, brush) {
            //We must delete the old brush
            let obj: HGDIOBJ = old.into();
            unsafe {
                let _ = DeleteObject(obj);
            }
        }
    }
}

pub struct GdiAppContext {
    rc: Rc<GdiResourceBook>
}
impl GraphicsAppContext for GdiAppContext {
    type Error = GdiError;
    type WindowContext = GdiWindowContext;
    
    fn make_window_context(&mut self, _over: HWND) -> Result<Self::WindowContext, Self::Error> {
        Ok ( GdiWindowContext {
            rc: Rc::clone(&self.rc)
        })
    }
}

pub struct GdiEngine;
impl GraphicsEngine for GdiEngine {
    type Error = GdiError;
    type AppContext = GdiAppContext;

    fn make_app_context(self) -> Result<Self::AppContext, Self::Error> {
        let rc = Rc::new(GdiResourceBook::default());
        Ok( 
            GdiAppContext {
                rc
            } 
        )
    }
}