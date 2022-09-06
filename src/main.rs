#![warn(absolute_paths_not_starting_with_crate)]
// #![warn(box_pointers)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(keyword_idents)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
// #![warn(missing_copy_implementations)]
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

mod plugins;

use bevy::{
    app::App,
    log::{debug, LogSettings},
    utils::tracing::Level,
};
#[cfg(feature = "render")]
use bevy::{
    asset::{AssetServer, Assets},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        change_detection::ResMut,
        component::Component,
        query::{Changed, With},
        system::{Commands, Query, Res},
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
        entity::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, FocusPolicy, Interaction, JustifyContent, PositionType, Size, Style, UiColor,
        UiImage, UiRect, Val,
    },
    utils::default,
    window::{CursorIcon, WindowDescriptor, Windows},
    winit::WinitSettings,
};
use plugins::WorldPlugins;
use save::*;

#[cfg(feature = "render")]
fn refresh_world_texture(images: &mut Assets<Image>, world_manager: &WorldManager) {
    debug!("refreshing world texture");
    let image_handle = images.get_handle(world_manager.image_handle_id);
    images.get_mut(&image_handle).unwrap().data = world_manager.world_color_bytes();
}

#[cfg(feature = "render")]
#[derive(Component, Default)]
struct RainfallButton;

#[cfg(feature = "render")]
#[derive(Component, Default)]
struct TemperatureButton;

#[cfg(feature = "render")]
#[derive(Component, Default)]
struct ContoursButton;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.60, 0.35);
#[cfg(feature = "render")]
fn handle_rainfall_button(
    mut interaction_query: Query<
        '_,
        '_,
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<RainfallButton>),
    >,
    mut windows: ResMut<'_, Windows>,
    mut images: ResMut<'_, Assets<Image>>,
    mut world_manager: ResMut<'_, WorldManager>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = PRESSED_BUTTON.into();
                debug!("Toggling rainfall");
                world_manager.toggle_rainfall();
                refresh_world_texture(&mut images, &world_manager)
            }
            Interaction::Hovered => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Hand);
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[cfg(feature = "render")]
fn handle_temperature_button(
    mut interaction_query: Query<
        '_,
        '_,
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<TemperatureButton>),
    >,
    mut windows: ResMut<'_, Windows>,
    mut images: ResMut<'_, Assets<Image>>,
    mut world_manager: ResMut<'_, WorldManager>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = PRESSED_BUTTON.into();
                debug!("Toggling temperature");
                world_manager.toggle_temperature();
                refresh_world_texture(&mut images, &world_manager)
            }
            Interaction::Hovered => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Hand);
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[cfg(feature = "render")]
fn handle_contours_button(
    mut interaction_query: Query<
        '_,
        '_,
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ContoursButton>),
    >,
    mut windows: ResMut<'_, Windows>,
    mut images: ResMut<'_, Assets<Image>>,
    mut world_manager: ResMut<'_, WorldManager>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = PRESSED_BUTTON.into();
                debug!("Toggling contours");
                world_manager.toggle_contours();
                refresh_world_texture(&mut images, &world_manager)
            }
            Interaction::Hovered => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Hand);
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[cfg(feature = "render")]
fn generate_graphics(
    mut commands: Commands<'_, '_>,
    mut images: ResMut<'_, Assets<Image>>,
    mut world_manager: ResMut<'_, WorldManager>,
    asset_server: Res<'_, AssetServer>,
) {
    let world = world_manager.get_world().unwrap();

    let image_handle = images.add(Image {
        data: world_manager.world_color_bytes(),
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
    world_manager.image_handle_id = image_handle.id;

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
        .with_children(|root_node| {
            _ = root_node.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    ..default()
                },
                image: UiImage(image_handle),
                ..default()
            });

            _ = root_node
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Undefined),
                        padding: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::SpaceAround,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|button_box| {
                    _ = button_box
                        .spawn_bundle(ButtonBundle {
                            button: Button,
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(RainfallButton::default())
                        .with_children(|button| {
                            _ = button.spawn_bundle(TextBundle {
                                text: bevy::text::Text::from_section(
                                    "Toggle rainfall",
                                    bevy::text::TextStyle {
                                        font: asset_server.load("JuliaMono.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                    _ = button_box
                        .spawn_bundle(ButtonBundle {
                            button: Button,
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(TemperatureButton::default())
                        .with_children(|button| {
                            _ = button.spawn_bundle(TextBundle {
                                text: bevy::text::Text::from_section(
                                    "Toggle temperature",
                                    bevy::text::TextStyle {
                                        font: asset_server.load("JuliaMono.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                    _ = button_box
                        .spawn_bundle(ButtonBundle {
                            button: Button,
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(ContoursButton::default())
                        .with_children(|button| {
                            _ = button.spawn_bundle(TextBundle {
                                text: bevy::text::Text::from_section(
                                    "Toggle contours",
                                    bevy::text::TextStyle {
                                        font: asset_server.load("JuliaMono.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                });
        });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mut manager = WorldManager::new();
    #[cfg(feature = "render")]
    {
        let world = manager.new_world()?;
        _ = app
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            // Use nearest-neighbor rendering for cripsier pixels
            .insert_resource(ImageSettings::default_nearest())
            .insert_resource(WindowDescriptor {
                width: (2 * world.width) as f32,
                height: (2 * world.height) as f32,
                title: String::from("World-RS"),
                resizable: true,
                ..default()
            })
            .add_startup_system(generate_graphics)
            .add_system(handle_rainfall_button)
            .add_system(handle_temperature_button)
            .add_system(handle_contours_button);
    }
    #[cfg(not(feature = "render"))]
    {
        _ = manager.new_world()?
    }

    #[cfg(feature = "debug")]
    {
        _ = app.insert_resource(LogSettings {
            level: Level::DEBUG,
            ..default()
        });
    }
    #[cfg(not(feature = "debug"))]
    {
        _ = app.insert_resource(LogSettings {
            level: Level::WARN,
            ..default()
        });
    }

    app.insert_resource(manager).add_plugins(WorldPlugins).run();

    Ok(())
}
