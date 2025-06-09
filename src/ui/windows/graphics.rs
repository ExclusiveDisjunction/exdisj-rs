use std::fmt::Display;

use windows::Win32::{Foundation::HWND};

use crate::ui::windows::rc::{ResolvedPenTemplate, ResourceBook};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ResourceKey {

}
impl Display for ResourceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub trait GraphicsFrameContext<G: ResourceBook> {
    type Error: std::error::Error;
    type Unit: PartialEq + PartialOrd;
    type Rect;

    fn bind_background(&mut self, color: Option<&G::Color>) -> Result<(), Self::Error>;
    fn bind_pen(&mut self, style: Option<ResolvedPenTemplate<'_, G::Color, Self::Unit>>) -> Result<(), Self::Error>;

    fn draw_rect(&mut self, size: Self::Rect, radius: Option<Self::Unit>, fill: bool) -> Result<(), Self::Error>;
    fn draw_circle(&mut self, radius: Self::Unit, center: (Self::Unit, Self::Unit)) -> Result<(), Self::Error> where Self::Unit: Clone{
        self.draw_ellipse(radius.clone(), radius, center)
    }
    fn draw_ellipse(&mut self, width: Self::Unit, height: Self::Unit, center: (Self::Unit, Self::Unit)) -> Result<(), Self::Error>;
    fn draw_line(&mut self, start: (Self::Unit, Self::Unit), to: (Self::Unit, Self::Unit)) -> Result<(), Self::Error>;
}

pub trait GraphicsViewContext<G: ResourceBook> {
    type Error: std::error::Error;
    type Frame: GraphicsFrameContext<G>;

    fn make_frame(&mut self) -> Result<Self::Frame, Self::Error>;
}
pub trait GraphicsWindowContext<G: ResourceBook> {
    type Error: std::error::Error;
    type ViewContext: GraphicsViewContext<G>;

    fn make_view_context(&mut self, over: HWND) -> Result<Self::ViewContext, Self::Error>;
}
pub trait GraphicsResourceBook {
    type Font;
    type Color;
    
    fn get_color(&self, key: &ResourceKey) -> Option<&Self::Color>;
    fn get_font(&self, key: &ResourceKey) -> Option<&Self::Font>;
}
pub trait GraphicsAppContext<G> {
    type Error: std::error::Error;
    type WindowContext: GraphicsWindowContext<G>;

    fn make_window_context(&mut self, over: HWND) -> Result<Self::WindowContext, Self::Error>;
}
pub trait GraphicsEngine {
    type Error: std::error::Error;
    type Resource: ResourceBook;
    type AppContext: GraphicsAppContext<Self::Resource>;

    fn make_app_context(self) -> Result<Self::AppContext, Self::Error>;
}