use std::fmt::Display;

use windows::Win32::Foundation::HWND;

use crate::ui::tool::utf_str::Utf16String;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ResourceKey {

}
impl Display for ResourceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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