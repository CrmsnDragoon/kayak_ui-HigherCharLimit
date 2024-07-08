use bevy::{
    math::UVec2,
    prelude::{Handle, Image},
    reflect::Reflect,
};
#[cfg(feature = "svg")]
use bevy_svg::prelude::Svg;
use kayak_font::{Alignment, TextLayout, TextProperties};

use super::Edge;

#[derive(Debug, Reflect, Clone, PartialEq)]
pub enum RenderCommand {
    Empty,
    /// Represents a node that has no renderable object but contributes to the layout.
    Layout,
    Clip,
    Quad,
    Text {
        content: String,
        alignment: Alignment,
        word_wrap: bool,
        subpixel: bool,
        text_layout: TextLayout,
        properties: TextProperties,
    },
    Image {
        handle: Handle<Image>,
    },
    TextureAtlas {
        position: UVec2,
        size: UVec2,
        handle: Handle<Image>,
    },
    NinePatch {
        border: Edge<f32>,
        handle: Handle<Image>,
        scale: f32,
    },
    #[cfg(feature = "svg")]
    Svg {
        handle: Handle<Svg>,
    },
}

impl Default for RenderCommand {
    fn default() -> Self {
        Self::Empty
    }
}
