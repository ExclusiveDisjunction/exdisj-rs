#[cfg(feature = "gdi_graphics")]
use windows::Win32::Foundation::COLORREF;

#[cfg(feature = "gdi_plus_graphics")]
use windows::Win32::Graphics::GdiPlus::Color as GdiColor;

#[cfg(feature = "direct_graphics")]
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;

#[cfg(feature = "direct3d_graphics")]
use windows::Win32::Graphics::Direct3D9::D3DCOLORVALUE;

use std::fmt::Display;

/// A graphics engine independent encoding a color. Colors can be represented with alpha. Values are stored from 0-255. 
/// Depending on features includes on the library, implementations of `Into` will be provided. 
/// For example, if the gdi_graphics feature is enabled, `Into<COLORREF>` will be provided. 
/// This type can also be used to hash, so it can be used in a `HashMap` as the key to a resolved resource.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorResource {
    data: [u8; 4] //red, green, blue, alpha
}
impl Display for ColorResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}{:X}{:X}{:X}", self.data[0], self.data[1], self.data[2], self.data[3])   
    }
}
#[cfg(feature = "gdi_graphics")]
impl Into<COLORREF> for ColorResource {
    fn into(self) -> COLORREF {
        let val = self.data[0] as u32 | (self.data[1] as u32) << 8 | (self.data[2] as u32) << 16;
        COLORREF(val)
    }
}
#[cfg(feature = "gdi_plus_graphics")]
impl Into<GdiColor> for ColorResource {
    fn into(self) -> GdiColor {
        let value = ((self.data[3] as u32) << 24)
              | ((self.data[0] as u32) << 16)
              | ((self.data[1] as u32) << 8)
              | (self.data[2] as u32);

       GdiColor { Argb: value }
    }
}
#[cfg(feature = "direct_graphics")]
impl Into<D2D1_COLOR_F> for ColorResource {
    fn into(self) -> D2D1_COLOR_F {
        let max = u8::MAX as f32;
        D2D1_COLOR_F { 
            r: self.data[0] as f32 / max, 
            g: self.data[1] as f32 / max,
            b: self.data[2] as f32 / max, 
            a: self.data[3] as f32 / max 
        }
    }
}
#[cfg(feature = "direct3d_graphics")]
impl Into<D3DCOLORVALUE> for ColorResource {
    fn into(self) -> D3DCOLORVALUE {
        let max = u8::MAX as f32;
        D3DCOLORVALUE { 
            r: self.data[0] as f32 / max, 
            g: self.data[1] as f32 / max, 
            b: self.data[2] as f32 / max, 
            a: self.data[3] as f32 / max 
        }
    }
}
impl From<[u8; 4]> for ColorResource {
    fn from(value: [u8; 4]) -> Self {
        Self {
            data: value
        }
    }
}
impl ColorResource {
    /// Creates a color around a specified values for RGBA. 
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            data: [red, green, blue, alpha]
        }
    }
}


pub struct FontStyle {

}

pub struct PenStyle<T> {
    width: T,
    color: ColorResource
}
impl<T> PenStyle<T> {
    pub fn new(color: ColorResource, width: T) -> Self {
        Self {
            width,
            color
        }
    }

    pub fn width(&self) -> &T {
        &self.width
    }
    pub fn set_width(&mut self, new: T) {
        self.width = new
    }
    pub fn color(&self) -> &ColorResource {
        &self.color
    }
    pub fn set_color(&mut self, new: ColorResource) {
        self.color = new
    }
}

pub enum StyleAccess {
    Background,
    Border,
    Accent,
    Foreground
}

pub struct StyleRequest {
    background: ColorResource,
    border: ColorResource,
    accent: ColorResource,
    foreground: ColorResource
}
impl StyleRequest {
    pub fn new(background: ColorResource, border: ColorResource, accent: ColorResource, foreground: ColorResource) -> Self {
        Self {
            background,
            border,
            accent,
            foreground
        }
    }

    pub fn get(&self, rc: StyleAccess) -> &ColorResource {
        match rc {
            StyleAccess::Accent => &self.accent,
            StyleAccess::Border => &self.border,
            StyleAccess::Foreground => &self.foreground,
            StyleAccess::Background=> &self.background
        }
    }
    pub fn get_mut(&mut self, rc: StyleAccess) -> &mut ColorResource {
        match rc {
            StyleAccess::Accent => &mut self.accent,
            StyleAccess::Border => &mut self.border,
            StyleAccess::Foreground => &mut self.foreground,
            StyleAccess::Background => &mut self.background
        }
    }
}