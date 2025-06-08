use std::fmt::Display;

use windows::Win32::{Foundation::HWND};
#[cfg(feature = "gdi_graphics")]
use windows::Win32::Foundation::COLORREF;

#[cfg(feature = "gdi_plus_graphics")]
use windows::Win32::Graphics::GdiPlus::Color as GdiColor;

#[cfg(feature = "direct_graphics")]
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;

#[cfg(feature = "direct3d_graphics")]
use windows::Win32::Graphics::Direct3D9::D3DCOLORVALUE;

use crate::ui::tool::utf_str::Utf16String;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ResourceKey {

}
impl Display for ResourceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct ColorResource {
    alpha: u8,
    red: u8,
    green: u8,
    blue: u8
}
#[cfg(feature = "gdi_graphics")]
impl Into<COLORREF> for ColorResource {
    fn into(self) -> COLORREF {
        let val = self.red as u32 | (self.green as u32) << 8 | (self.blue as u32) << 16;
        COLORREF(val)
    }
}
#[cfg(feature = "gdi_plus_graphics")]
impl Into<GdiColor> for ColorResource {
    fn into(self) -> GdiColor {
        let value = ((self.alpha as u32) << 24)
              | ((self.red as u32) << 16)
              | ((self.green as u32) << 8)
              | (self.blue as u32);

       GdiColor { Argb: value }
    }
}
#[cfg(feature = "direct_graphics")]
impl Into<D2D1_COLOR_F> for ColorResource {
    fn into(self) -> D2D1_COLOR_F {
        let max = u8::MAX as f32;
        D2D1_COLOR_F { 
            g: self.green as f32 / max,
            r: self.red   as f32 / max, 
            b: self.blue  as f32 / max, 
            a: self.alpha as f32 / max 
        }
    }
}
#[cfg(feature = "direct3d_graphics")]
impl Into<D3DCOLORVALUE> for ColorResource {
    fn into(self) -> D3DCOLORVALUE {
        let max = u8::MAX as f32;
        D3DCOLORVALUE { 
            r: self.red as f32   / max, 
            g: self.green as f32 / max, 
            b: self.blue as f32  / max, 
            a: self.alpha as f32 / max 
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Backgrounds {
    Primary,
    Secondary, 
    Tertiary
}

pub enum StyleAccess {
    Background(Backgrounds),
    Accent,
    Foreground
}

pub trait StyleBook {
    type Storage;

    fn get(&self, rc: StyleAccess) -> &Self::Storage;
    fn get_mut(&mut self, rc: StyleAccess) -> &mut Self::Storage;
}

pub struct StyleTemplate {
    id: u32,
    background: [ColorResource; 3],
    accent: ColorResource,
    foreground: ColorResource
}
impl StyleBook for StyleTemplate {
    type Storage = ColorResource;

    fn get(&self, rc: StyleAccess) -> &ColorResource {
        match rc {
            StyleAccess::Accent => &self.accent,
            StyleAccess::Foreground => &self.foreground,
            StyleAccess::Background(b) => &self.background[b as u8 as usize]
        }
    }
    fn get_mut(&mut self, rc: StyleAccess) -> &mut ColorResource {
        match rc {
            StyleAccess::Accent => &mut self.accent,
            StyleAccess::Foreground => &mut self.foreground,
            StyleAccess::Background(b) => &mut self.background[b as u8 as usize]
        }
    }
}
impl StyleTemplate {
    pub fn new(id: u32, background: [ColorResource; 3], accent: ColorResource, foreground: ColorResource) -> Self {
        Self {
            id,
            background,
            accent,
            foreground
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

pub struct FontStyle {

}

pub trait GraphicsFrameContext {
    type Error: std::error::Error;
    type Unit: PartialEq + PartialOrd;
    type Rect;

    fn draw_bk(&mut self, color: ResourceKey) -> Result<(), Self::Error>;
    fn draw_rect(&mut self, size: Self::Rect, radius: Option<Self::Unit>, color: ResourceKey) -> Result<(), Self::Error>;
    fn draw_frame(&mut self, size: Self::Rect, radius: Option<Self::Unit>, thickness: Self::Unit, color: ResourceKey, bk: ResourceKey) -> Result<(), Self::Error>;
    fn draw_circle(&mut self, radius: Self::Unit, x: Self::Unit, y: Self::Unit, color: ResourceKey) -> Result<(), Self::Error>;
    
    fn write_text(&mut self, text: &Utf16String, x: Self::Unit, y: Self::Unit, style: FontStyle) -> Result<(), Self::Error>;
}

pub trait GraphicsViewContext {
    type Error: std::error::Error;
    type Frame: GraphicsFrameContext;

    fn make_frame(&mut self) -> Result<Self::Frame, Self::Error>;
}
pub trait GraphicsWindowContext {
    type Error: std::error::Error;
    type ViewContext: GraphicsViewContext;

    fn make_view_context(&mut self, over: HWND) -> Result<Self::ViewContext, Self::Error>;
}
pub trait GraphicsResourceBook {
    type Font;
    type Color;
    
    fn get_color(&self, key: &ResourceKey) -> Option<&Self::Color>;
    fn get_font(&self, key: &ResourceKey) -> Option<&Self::Font>;
}
pub trait GraphicsAppContext {
    type Error: std::error::Error;
    type WindowContext: GraphicsWindowContext;

    fn make_window_context(&mut self, over: HWND) -> Result<Self::WindowContext, Self::Error>;
}
pub trait GraphicsEngine {
    type Error: std::error::Error;
    type AppContext: GraphicsAppContext;

    fn make_app_context(self) -> Result<Self::AppContext, Self::Error>;
}