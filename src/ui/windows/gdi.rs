use std::rc::Rc;
use std::{collections::HashMap, fmt::Display};
use std::mem::MaybeUninit;

use windows::Win32::Foundation::COLORREF;
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, DeleteObject, RoundRect, SetBkMode, HGDIOBJ, TRANSPARENT};
use windows::Win32::{Foundation::{HWND, RECT}, Graphics::Gdi::{BeginPaint, EndPaint, FillRect, HBRUSH, HDC, HFONT, PAINTSTRUCT}, UI::WindowsAndMessaging::GetWindowRect};

use crate::ui::windows::rc::ResourceBook;

use super::graphics::{ResourceKey, GraphicsFrameContext, GraphicsViewContext, GraphicsWindowContext, GraphicsAppContext, GraphicsEngine, GraphicsResourceBook};
use super::style::{ColorResource, PenStyle};
//use crate::ui::tool::utf_str::Utf16String;

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
    wnd: HWND
}
impl GraphicsFrameContext for GdiFrameContext {
    type Error = GdiError;
    type Unit = i32;
    type Rect = RECT;
    
    fn bind_background(&mut self, color: Option<ColorResource>) -> Result<(), Self::Error> {
        todo!()
    }
    fn bind_pen(&mut self, style: Option<PenStyle<Self::Unit>>) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn draw_rect(&mut self, size: Self::Rect, radius: Option<Self::Unit>, fill: bool) -> Result<(), Self::Error> {
        todo!()
    }
    fn draw_ellipse(&mut self, width: Self::Unit, height: Self::Unit, center: (Self::Unit, Self::Unit)) -> Result<(), Self::Error> {
        todo!()
    }
    fn draw_line(&mut self, start: (Self::Unit, Self::Unit), to: (Self::Unit, Self::Unit)) -> Result<(), Self::Error> {
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
impl GraphicsViewContext<GdiResourceBook> for GdiViewContext {
    type Error = GdiError;
    type Frame = GdiFrameContext;

    fn make_frame(&mut self) -> Result<Self::Frame, Self::Error> {
        let ps: PAINTSTRUCT;
        let dc: HDC;

        unsafe {
            let mut ps_box: Box<MaybeUninit<PAINTSTRUCT>> = Box::new_uninit();

            dc = BeginPaint(self.target, ps_box.as_mut_ptr());
            ps = *ps_box.assume_init();

            SetBkMode(dc, TRANSPARENT);
        }

        Ok(
            Self::Frame {
                rc: Rc::clone(&self.rc),
                ps,
                hdc: dc,
                wnd: self.target
            }
        )
    }
}

pub struct GdiWindowContext {
    rc: Rc<GdiResourceBook>
}
impl GraphicsWindowContext<GdiResourceBook> for GdiWindowContext {
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
impl ResourceBook for GdiResourceBook {
    type Color = HBRUSH;
    type Font = HFONT;
    type Error = GdiError;
    
    fn make_from_template(&mut self, style: super::style::StyleRequest) -> Result<(), Self::Error> {
        todo!()
    }
    fn make_color(&mut self, key: ColorResource) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn remove_color(&mut self, key: ColorResource) -> bool {
        todo!()
    }
    
    fn get_special(&self, key: super::rc::SpecialColors) -> Option<&Self::Color> {
        todo!()
    }
    
    fn get(&self, key: &ColorResource) -> Option<&Self::Color> {
        todo!()
    }
    fn get_mut(&mut self, key: &ColorResource) -> Option<&Self::Font> {
        todo!()
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
impl GraphicsAppContext<GdiResourceBook> for GdiAppContext {
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
    type Resource = GdiResourceBook;

    fn make_app_context(self) -> Result<Self::AppContext, Self::Error> {
        let rc = Rc::new(GdiResourceBook::default());
        Ok( 
            GdiAppContext {
                rc
            } 
        )
    }
}