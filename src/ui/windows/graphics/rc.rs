use super::style::{StyleAccess, ColorResource, StyleRequest};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SpecialColors {
    Clear,
    Black,
    Red,
    Green,
    Blue,
    White
}

pub trait ResourceBook {
    type Brush;
    type Pen;
    type Font;
    type Error;

    fn include_style(&mut self, style: StyleRequest) -> Result<(), Self::Error>;
    fn get_style<'a>(&'a self, style: &StyleRequest) -> Option<ResolvedStyle<'a, Self::Brush>> where Self::Brush: 'a {
        let bk = self.get(style.get(StyleAccess::Background))?;
        let border = self.get(style.get(StyleAccess::Border))?;
        let accent = self.get(style.get(StyleAccess::Accent))?;
        let fore = self.get(style.get(StyleAccess::Foreground))?;

        Some(
            ResolvedStyle { 
                background: bk, 
                border, 
                accent, 
                foreground: fore 
            }
        )
    }

    fn make_color(&mut self, key: ColorResource) -> Result<(), Self::Error>;
    fn remove_color(&mut self, key: ColorResource) -> bool;

    fn get_special(&self, key: SpecialColors) -> Option<&Self::Brush>;

    fn get(&self, key: &ColorResource) -> Option<&Self::Brush>;
    fn get_mut(&mut self, key: &ColorResource) -> Option<&Self::Font>;

    fn make_pen<U>(&mut self, key: ColorResource, size: U) -> Result<(), Self::Error>;
    fn get_pen<U>(&self, key: ColorResource, size: U) -> Option<&Self::Pen>;
}

/// A color guide provides the ability for someone to have a basis around a set of colors.
/// This should be used inside of a `ResolvedStyle` structure to store basic color information.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolveColorGuide<'a, C> where C: 'a {
    background: [&'a C; 3],
    accent: &'a C,
    foreground: &'a C
}

/// A per-view structure used to build a borrowed set of colors.
/// These resolved colors should be able to be directly used by the graphics pipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedStyle<'a, C> where C: 'a {
    background: &'a C,
    border: &'a C,
    accent: &'a C,
    foreground: &'a C
}