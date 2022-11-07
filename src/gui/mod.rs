pub(crate) mod widget;
pub(crate) use widget::*;
pub(crate) mod window;
pub(crate) use window::*;

pub(crate) mod widgets;
pub(crate) mod windows;

use {
    crate::gui::{open_window, WidgetId, WidgetSystem},
    bevy::{
        asset::Assets,
        ecs::change_detection::Mut,
        log::debug,
        render::{render_resource::Extent3d, texture::Image},
    },
    planet::WorldManager,
};

fn update_textures(world_manager: &WorldManager, images: &mut Mut<Assets<Image>>) {
    debug!("refreshing world texture");
    let map_image_handle = images.get_handle(
        world_manager
            .render_settings
            .map_image_handle_id
            .expect("No map image handle"),
    );
    let map_image = images
        .get_mut(&map_image_handle)
        .expect("Map image handle pointing to non-existing image");
    map_image.resize(Extent3d {
        width:                 world_manager.world().width,
        height:                world_manager.world().height,
        depth_or_array_layers: 1,
    });
    map_image.data = world_manager.map_color_bytes();
}
