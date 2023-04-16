use crate::{
    render::unified::pipeline::{ExtractedQuad, UIQuadType},
    render_primitive::RenderPrimitive,
    styles::Corner,
};
use bevy::{
    math::Vec2,
    prelude::{Entity, Rect},
    render::color::Color,
};

pub fn extract_images(
    camera_entity: Entity,
    render_command: &RenderPrimitive,
    _dpi: f32,
) -> Vec<ExtractedQuad> {
    let (border_radius, layout, handle) = match render_command {
        RenderPrimitive::Image {
            border_radius,
            layout,
            handle,
        } => (*border_radius, layout, handle),
        _ => panic!(""),
    };

    vec![ExtractedQuad {
        camera_entity,
        rect: Rect {
            min: Vec2::new(layout.posx, layout.posy),
            max: Vec2::new(layout.posx + layout.width, layout.posy + layout.height),
        },
        color: Color::WHITE,
        char_id: 0,
        z_index: layout.z_index,
        font_handle: None,
        quad_type: UIQuadType::Image,
        type_index: 0,
        border_radius: Corner {
            top_left: border_radius.top_left,
            top_right: border_radius.top_right,
            bottom_left: border_radius.bottom_left,
            bottom_right: border_radius.bottom_right,
        },
        image: Some(handle.clone_weak()),
        uv_max: None,
        uv_min: None,
    }]
}
