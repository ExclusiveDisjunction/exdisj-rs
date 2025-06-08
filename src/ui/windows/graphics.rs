use std::fmt::Display;

use windows::Win32::{Foundation::HWND};

use crate::ui::{tool::utf_str::Utf16String, windows::style::ColorResource};

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

pub struct PenStyle<T> {
    width: T,
    color: ColorResource
}

pub trait GraphicsFrameContext {
    type Error: std::error::Error;
    type Unit: PartialEq + PartialOrd;
    type Rect;

    fn bind_background(&mut self, color: ColorResource) -> Result<(), Self::Error>;
    fn bind_pen(&mut self, style: PenStyle) -> Result<(), Self::Error>;

    fn draw_rect(&mut self, size: Self::Rect, radius: Option<Self::Unit>, fill: bool) -> Result<(), Self::Error>;
    fn draw_circle(&mut self, radius: Self::Unit, center: (Self::Unit, Self::Unit)) -> Result<(), Self::Error> where Self::Unit: Clone{
        self.draw_ellipse(radius.clone(), radius, center)
    }
    fn draw_ellipse(&mut self, width: Self::Unit, height: Self::Unit, center: (Self::Unit, Self::Unit)) -> Result<(), Self::Error>;
    fn draw_line(&mut self, start: (Self::Unit, Self::Unit), to: (Self::Unit, Self::Unit)) -> Result<(), Self::Error>;
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