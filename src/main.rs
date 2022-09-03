#![warn(absolute_paths_not_starting_with_crate)]
// #![warn(box_pointers)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(keyword_idents)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// #![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(noop_method_call)]
#![warn(pointer_structural_match)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rust_2021_prefixes_incompatible_syntax)]
#![warn(rust_2021_prelude_collisions)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unstable_features)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

use bevy::{
    app::App,
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::{
        change_detection::ResMut,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::{
        color::Color,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{Image, ImageSettings},
    },
    ui::{
        entity::{ImageBundle, NodeBundle},
        Size, Style, UiImage, Val,
    },
    utils::default,
    window::WindowDescriptor,
    winit::WinitSettings,
    DefaultPlugins,
};
use save::*;

fn get_color(cell: &TerrainCell) -> Color {
    let altitude_color = gen_altitude_color(cell.altitude);
    let rainfall_color = gen_rainfall_color(cell.rainfall);

    let normalized_rainfall = f32::max(cell.rainfall / World::MAX_RAINFALL, 0.0);

    let red = (altitude_color.r() * (1.0 - normalized_rainfall))
        + rainfall_color.r() * normalized_rainfall;
    let green = (altitude_color.g() * (1.0 - normalized_rainfall))
        + rainfall_color.g() * normalized_rainfall;
    let blue = (altitude_color.b() * (1.0 - normalized_rainfall))
        + rainfall_color.b() * normalized_rainfall;

    Color::rgb(red, green, blue)
}

fn gen_altitude_color(altitude: f32) -> Color {
    if altitude < 0.0 {
        Color::BLUE
    } else {
        let mult = (altitude - World::MIN_ALTITUDE) / World::ALTITUDE_SPAN;

        Color::rgb(0.58 * mult, 0.29 * mult, 0.0)
    }
}

fn gen_rainfall_color(rainfall: f32) -> Color {
    if rainfall < 0.0 {
        Color::BLACK
    } else {
        let mult = rainfall / World::MAX_RAINFALL;
        Color::GREEN * mult
    }
}

fn generate_texture(
    mut commands: Commands<'_, '_>,
    mut images: ResMut<'_, Assets<Image>>,
    world_manager: Res<'_, WorldManager>,
) {
    let world = world_manager.get_world().unwrap();
    let terrain_cells: Vec<_> = world.terrain.iter().rev().flatten().collect();
    let colors: Vec<_> = terrain_cells.iter().map(|cell| get_color(cell)).collect();
    let data: Vec<_> = colors
        .iter()
        .flat_map(|color| {
            color
                .as_rgba_f32()
                .iter()
                .flat_map(|num| num.to_le_bytes())
                .collect::<Vec<u8>>()
        })
        .collect();

    let image_handle = images.add(Image {
        data,
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: world.width as u32,
                height: world.height as u32,
                ..default()
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        },
        ..default()
    });

    _ = commands.spawn_bundle(Camera2dBundle::default());
    _ = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            _ = parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                image: UiImage(image_handle),
                ..default()
            });
            // });
        });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = WorldManager::new();
    let world = manager.new_world()?;

    App::new()
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        // Use nearest-neighbor rendering for cripsier pixels
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: world.width as f32,
            height: world.height as f32,
            title: String::from("World-RS"),
            resizable: true,
            ..default()
        })
        .insert_resource(manager)
        .add_startup_system(generate_texture)
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();

    Ok(())
}
