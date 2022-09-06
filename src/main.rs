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

use std::fmt::Display;

use bevy::{
    app::App,
    log::{debug, LogSettings},
    utils::tracing::Level,
};
#[cfg(feature = "render")]
use bevy::{
    asset::{AssetServer, Assets, Handle},
    core_pipeline::{
        core_2d::{Camera2d, Camera2dBundle},
        core_3d::{Camera3d, Camera3dBundle},
    },
    ecs::{
        change_detection::ResMut,
        component::Component,
        query::{Changed, With},
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    prelude::{Vec2, Vec3},
    render::{
        camera::{Camera, OrthographicProjection, RenderTarget},
        color::Color,
        mesh::{shape::Icosphere, Mesh},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{Image, ImageSettings},
    },
    sprite::{Sprite, SpriteBundle},
    text::Text,
    transform::components::{GlobalTransform, Transform},
    ui::{
        entity::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, AlignSelf, FocusPolicy, Interaction, JustifyContent, PositionType, Size, Style,
        UiColor, UiRect, Val,
    },
    utils::default,
    window::{CursorIcon, WindowDescriptor, Windows},
    winit::WinitSettings,
};
#[cfg(feature = "render")]
use plugins::PanCam;
use plugins::WorldPlugins;
use save::*;

#[cfg(feature = "render")]
fn refresh_world_texture(images: &mut Assets<Image>, world_manager: &WorldManager) {
    debug!("refreshing world texture");
    let image_handle = images.get_handle(world_manager.image_handle_id);
    images.get_mut(&image_handle).unwrap().data = world_manager.world_color_bytes();

    // TODO: Update Icosphere material... try to find out why it doesn't automatically=
}

#[cfg(feature = "render")]
#[derive(Component)]
struct RainfallButton;

#[cfg(feature = "render")]
#[derive(Component)]
struct TemperatureButton;

#[cfg(feature = "render")]
#[derive(Component)]
struct ContoursButton;

#[cfg(feature = "render")]
#[derive(Component)]
struct InfoPanel;

#[cfg(feature = "render")]
#[derive(Default, Debug)]
struct CursorMapPosition {
    x: i32,
    y: i32,
}
impl Display for CursorMapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x: {}, y: {}", self.x, self.y))
    }
}

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
fn update_cursor_map_position(
    mut cursor_map_position: ResMut<'_, CursorMapPosition>,
    transform: Query<'_, '_, (&Camera, &GlobalTransform), With<Camera2d>>,
    windows: Res<'_, Windows>,
    world_manager: Res<'_, WorldManager>,
) {
    let (camera, transform) = transform.single();

    let window = match camera.target {
        RenderTarget::Window(window_id) => windows.get(window_id).unwrap(),
        RenderTarget::Image(_) => windows.primary(),
    };

    if let Some(screen_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());

        // GPU coordinates [-1..1]
        let ndc = (screen_position / window_size) * 2.0;

        // Matrix to reverse camera transform
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_position =
            ndc_to_world.project_point3(ndc.extend(-1.0)).truncate() / WORLD_SCALE as f32;

        cursor_map_position.x = world_position.x.round() as i32;
        cursor_map_position.y = world_manager.world().height - world_position.y.round() as i32;
    }
}

const ROTATION_SPEED: f32 = 0.002;
#[cfg(feature = "render")]
fn rotate_planet(mut planet_transform: Query<'_, '_, &mut Transform, With<Handle<Mesh>>>) {
    planet_transform.single_mut().rotate_y(ROTATION_SPEED);
}

#[cfg(feature = "render")]
fn update_info_panel(
    cursor_position: Res<'_, CursorMapPosition>,
    world_manager: Res<'_, WorldManager>,
    mut text: Query<'_, '_, &mut Text, With<InfoPanel>>,
) {
    let world = world_manager.world();
    text.single_mut().sections[0].value = if cursor_position.x >= 0
        && cursor_position.x < world.width
        && cursor_position.y >= 0
        && cursor_position.y < world.height
    {
        let cell = &world.terrain[cursor_position.y as usize][cursor_position.x as usize];
        format!(
            "Mouse position: {}\nAltitude: {}\nRainfall: {}\nTemperature: {}",
            *cursor_position, cell.altitude, cell.rainfall, cell.temperature
        )
    } else {
        format!("Mouse position: {}\nOut of bounds", *cursor_position)
    };
}

#[cfg(feature = "render")]
fn generate_graphics(
    mut commands: Commands<'_, '_>,
    mut images: ResMut<'_, Assets<Image>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut world_manager: ResMut<'_, WorldManager>,
    asset_server: Res<'_, AssetServer>,
) {
    let world = world_manager.world();
    let custom_sprite_size = Vec2 {
        x: (WORLD_SCALE * world.width) as f32,
        y: (WORLD_SCALE * world.height) as f32,
    };

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

    _ = commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(default(), Vec3::Y),
        projection: OrthographicProjection {
            scale: 0.01,
            ..default()
        }
        .into(),
        ..default()
    });
    _ = commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Icosphere {
            radius: 2.0,
            subdivisions: 9,
        })),
        material: materials.add(images.get_handle(world_manager.image_handle_id).into()),
        transform: Transform::from_translation(default()),
        ..default()
    });
    _ = commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(-20.0, 20.0, 50.0),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..default()
        },
        ..default()
    });

    _ = commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        })
        .insert(PanCam::default());
    _ = commands.spawn_bundle(SpriteBundle {
        texture: images.get_handle(world_manager.image_handle_id),
        sprite: Sprite {
            custom_size: Some(custom_sprite_size),
            ..default()
        },
        ..default()
    });
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
            _ = root_node
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        padding: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgba(1.0, 1.0, 1.0, 0.05).into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|info_panel| {
                    _ = info_panel
                        .spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "Info Panel",
                                bevy::text::TextStyle {
                                    font: asset_server.load("JuliaMono.ttf"),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        })
                        .insert(InfoPanel);
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
                        .insert(RainfallButton)
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
                        .insert(TemperatureButton)
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
                        .insert(ContoursButton)
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

const WORLD_SCALE: i32 = 3;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mut manager = WorldManager::new();
    #[cfg(feature = "render")]
    {
        let world = manager.new_world()?;
        _ = app
            .insert_resource(WinitSettings::game())
            // Use nearest-neighbor rendering for cripsier pixels
            .insert_resource(ImageSettings::default_nearest())
            .insert_resource(WindowDescriptor {
                width: (WORLD_SCALE * world.width) as f32,
                height: (WORLD_SCALE * world.height) as f32,
                title: String::from("World-RS"),
                resizable: true,
                ..default()
            })
            .insert_resource(CursorMapPosition::default())
            .add_startup_system(generate_graphics)
            .add_system(handle_rainfall_button)
            .add_system(handle_temperature_button)
            .add_system(handle_contours_button)
            .add_system(update_cursor_map_position)
            .add_system(update_info_panel)
            .add_system(rotate_planet);
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

    app.add_plugins(WorldPlugins).insert_resource(manager).run();

    Ok(())
}
