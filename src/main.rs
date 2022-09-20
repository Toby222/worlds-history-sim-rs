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

mod components;
mod plugins;
mod resources;
mod ui_helpers;

#[cfg(all(feature = "debug", feature = "render"))]
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    log::debug,
};
#[cfg(feature = "render")]
use {
    bevy::text::Font,
    bevy::{
        asset::Assets,
        core_pipeline::core_2d::{Camera2d, Camera2dBundle},
        ecs::{
            change_detection::ResMut,
            query::{Changed, With},
            system::{Commands, Query, Res},
        },
        hierarchy::BuildChildren,
        prelude::Vec2,
        render::{
            camera::{Camera, RenderTarget},
            color::Color,
            render_resource::{
                Extent3d,
                TextureDescriptor,
                TextureDimension,
                TextureFormat,
                TextureUsages,
            },
            texture::{Image, ImageSettings},
        },
        sprite::{Sprite, SpriteBundle},
        text::Text,
        transform::components::GlobalTransform,
        ui::{
            entity::{NodeBundle, TextBundle},
            AlignSelf,
            FocusPolicy,
            Interaction,
            JustifyContent,
            PositionType,
            Size,
            Style,
            UiColor,
            UiRect,
            Val,
        },
        window::{CursorIcon, WindowDescriptor, Windows},
        winit::WinitSettings,
    },
    components::{
        markers::{InfoPanel, ToolbarButton},
        third_party::PanCam,
    },
    planet::BiomeStats,
    resources::CursorMapPosition,
    ui_helpers::{toolbar_button, toolbar_button_text},
};
use {
    bevy::{
        app::App,
        log::LogSettings,
        utils::{default, tracing::Level},
    },
    planet::WorldManager,
    plugins::WorldPlugins,
};
#[cfg(all(feature = "render", feature = "planet_view"))]
use {
    bevy::{
        asset::Handle,
        core_pipeline::core_3d::{Camera3d, Camera3dBundle},
        ecs::query::Without,
        pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
        prelude::{Quat, Vec3},
        render::camera::OrthographicProjection,
        render::mesh::{shape::UVSphere, Mesh},
        transform::components::Transform,
    },
    std::f32::consts::FRAC_PI_2,
};

#[cfg(feature = "render")]
fn refresh_map_texture(
    images: &mut Assets<Image>,
    #[cfg(feature = "planet_view")] materials: &mut Assets<StandardMaterial>,
    world_manager: &WorldManager,
) {
    let world = world_manager.world();
    #[cfg(feature = "debug")]
    debug!("refreshing world texture");
    let map_image_handle = images.get_handle(
        world_manager
            .map_image_handle_id
            .expect("No map image handle"),
    );
    let map_image = images
        .get_mut(&map_image_handle)
        .expect("Map image handle pointing to non-existing image");
    map_image.resize(Extent3d {
        width:                 world.width,
        height:                world.height,
        depth_or_array_layers: 1,
    });
    map_image.data = world_manager.map_color_bytes();

    #[cfg(feature = "planet_view")]
    {
        let planet_image_handle = images.get_handle(
            world_manager
                .planet_image_handle_id
                .expect("No planet image handle"),
        );
        let planet_image = images
            .get_mut(&planet_image_handle)
            .expect("Planet image handle pointing to non-existing image");
        planet_image.resize(Extent3d {
            width:                 world.width,
            height:                world.height,
            depth_or_array_layers: 1,
        });
        planet_image.data = world_manager.planet_color_bytes();

        let planet_material_handle = materials.get_handle(
            world_manager
                .planet_material_handle_id
                .expect("No planet material handle"),
        );
        let planet_material = materials
            .get_mut(&planet_material_handle)
            .expect("Planet material handle pointing to non-existing material");
        planet_material.base_color_texture = Some(planet_image_handle);
    }
}

#[cfg(feature = "render")]
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
#[cfg(feature = "render")]
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
#[cfg(feature = "render")]
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.60, 0.35);
#[cfg(feature = "render")]
fn handle_toolbar_button(
    mut interaction_query: Query<
        '_,
        '_,
        (&Interaction, &mut UiColor, &ToolbarButton),
        Changed<Interaction>,
    >,
    mut windows: ResMut<'_, Windows>,
    mut images: ResMut<'_, Assets<Image>>,
    mut world_manager: ResMut<'_, WorldManager>,
    #[cfg(feature = "planet_view")] mut camera_3d_query: Query<
        '_,
        '_,
        &mut Camera,
        (With<Camera3d>, Without<Camera2d>),
    >,
    #[cfg(feature = "planet_view")] mut camera_2d_query: Query<
        '_,
        '_,
        &mut Camera,
        (With<Camera2d>, Without<Camera3d>),
    >,
    #[cfg(feature = "planet_view")] mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    for (interaction, mut color, toolbar_button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = PRESSED_BUTTON.into();
                match toolbar_button {
                    ToolbarButton::Rainfall => {
                        #[cfg(feature = "debug")]
                        debug!("Toggling rainfall");
                        world_manager.toggle_rainfall();
                        refresh_map_texture(
                            &mut images,
                            #[cfg(feature = "planet_view")]
                            &mut materials,
                            &world_manager,
                        );
                    },
                    ToolbarButton::Temperature => {
                        #[cfg(feature = "debug")]
                        debug!("Toggling temperature");
                        world_manager.toggle_temperature();
                        refresh_map_texture(
                            &mut images,
                            #[cfg(feature = "planet_view")]
                            &mut materials,
                            &world_manager,
                        );
                    },
                    ToolbarButton::Biomes => {
                        #[cfg(feature = "debug")]
                        debug!("Toggling biomes");
                        world_manager.toggle_biomes();
                        refresh_map_texture(
                            &mut images,
                            #[cfg(feature = "planet_view")]
                            &mut materials,
                            &world_manager,
                        );
                    },
                    ToolbarButton::Contours => {
                        #[cfg(feature = "debug")]
                        debug!("Toggling contours");
                        world_manager.toggle_contours();
                        refresh_map_texture(
                            &mut images,
                            #[cfg(feature = "planet_view")]
                            &mut materials,
                            &world_manager,
                        );
                    },
                    ToolbarButton::GenerateWorld => {
                        #[cfg(feature = "debug")]
                        debug!("Generating new world");
                        _ = world_manager
                            .new_world()
                            .expect("Failed to generate new world");
                        refresh_map_texture(
                            &mut images,
                            #[cfg(feature = "planet_view")]
                            &mut materials,
                            &world_manager,
                        );
                    },
                    ToolbarButton::SaveWorld => {
                        #[cfg(feature = "debug")]
                        debug!("Saving world");
                        _ = world_manager.save_world("planet.ron");
                    },
                    ToolbarButton::LoadWorld => {
                        #[cfg(feature = "debug")]
                        debug!("Loading world");
                        _ = world_manager.load_world("planet.ron", &mut images);
                        refresh_map_texture(
                            &mut images,
                            #[cfg(feature = "planet_view")]
                            &mut materials,
                            &world_manager,
                        );
                    },
                    #[cfg(feature = "planet_view")]
                    ToolbarButton::PlanetView => {
                        #[cfg(feature = "debug")]
                        debug!("Toggling planet view");
                        let mut camera_3d = camera_3d_query.single_mut();
                        camera_3d.is_active = !camera_3d.is_active;
                        let mut camera_2d = camera_2d_query.single_mut();
                        camera_2d.is_active = !camera_2d.is_active;
                    },
                }
            },
            Interaction::Hovered => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Hand);
                *color = HOVERED_BUTTON.into();
            },
            Interaction::None => {
                windows.primary_mut().set_cursor_icon(CursorIcon::Default);
                *color = NORMAL_BUTTON.into();
            },
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
        let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE;

        // Matrix to reverse camera transform
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_position =
            ndc_to_world.project_point3(ndc.extend(-1.0)).truncate() / WORLD_SCALE as f32;

        let world = world_manager.world();
        cursor_map_position.x = world.width as i32 / 2 + f32::ceil(world_position.x) as i32 - 1;
        cursor_map_position.y = world.height as i32 / 2 + f32::ceil(world_position.y) as i32 - 1;
    }
}

#[cfg(all(feature = "render", feature = "planet_view"))]
const ROTATION_SPEED: f32 = 0.002;
#[cfg(all(feature = "render", feature = "planet_view"))]
fn rotate_planet(mut planet_transform: Query<'_, '_, &mut Transform, With<Handle<Mesh>>>) {
    planet_transform.single_mut().rotate_y(ROTATION_SPEED);
}

#[cfg(feature = "render")]
fn update_info_panel(
    #[cfg(feature = "debug")] diagnostics: Res<'_, Diagnostics>,
    cursor_position: Res<'_, CursorMapPosition>,
    world_manager: Res<'_, WorldManager>,
    mut text: Query<'_, '_, &mut Text, With<InfoPanel>>,
) {
    let world = world_manager.world();
    text.single_mut().sections[0].value = if cursor_position.x >= 0
        && cursor_position.x < world.width as i32
        && cursor_position.y >= 0
        && cursor_position.y < world.height as i32
    {
        let cell = &world.terrain[cursor_position.y as usize][cursor_position.x as usize];

        #[cfg(feature = "debug")]
        {
            format!(
                "FPS: ~{}\nMouse position: {}\nAltitude: {}\nRainfall: {}\nTemperature: {}\n\n{}",
                match diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
                    None => f64::NAN,
                    Some(fps) => fps.value.round(),
                },
                *cursor_position,
                cell.altitude,
                cell.rainfall,
                cell.temperature,
                cell.biome_presences
                    .iter()
                    .map(|(biome_type, presence)| {
                        format!(
                            "Biome: {} ({:.2}%)",
                            (<BiomeStats>::from(biome_type).name),
                            presence * 100.0
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }

        #[cfg(not(feature = "debug"))]
        {
            format!(
                "Mouse position: {}\nAltitude: {}\nRainfall: {}\nTemperature: {}\n{}",
                *cursor_position,
                cell.altitude,
                cell.rainfall,
                cell.temperature,
                cell.biome_presences
                    .iter()
                    .map(|(biome_type, presence)| {
                        format!(
                            "Biome: {} ({:.2}%)",
                            (<BiomeStats>::from(biome_type).name),
                            presence * 100.0
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
    } else {
        #[cfg(feature = "debug")]
        {
            format!(
                "FPS: ~{}\nMouse position: {}\nOut of bounds",
                match diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
                    None => f64::NAN,
                    Some(fps) => fps.value.round(),
                },
                *cursor_position
            )
        }

        #[cfg(not(feature = "debug"))]
        {
            format!("Mouse position: {}\nOut of bounds", *cursor_position)
        }
    };
}

#[cfg(feature = "render")]
fn generate_graphics(
    mut commands: Commands<'_, '_>,
    mut world_manager: ResMut<'_, WorldManager>,
    mut images: ResMut<'_, Assets<Image>>,
    mut fonts: ResMut<'_, Assets<Font>>,
    #[cfg(feature = "planet_view")] mut materials: ResMut<'_, Assets<StandardMaterial>>,
    #[cfg(feature = "planet_view")] mut meshes: ResMut<'_, Assets<Mesh>>,
) {
    let julia_mono_handle = fonts.add(
        Font::try_from_bytes(include_bytes!("../assets/JuliaMono.ttf").to_vec())
            .expect("Failed to create JuliaMono font!"),
    );
    let world = world_manager.world();
    let custom_sprite_size = Vec2 {
        x: (WORLD_SCALE * world.width as i32) as f32,
        y: (WORLD_SCALE * world.height as i32) as f32,
    };

    let map_image_handle = images.add(Image {
        data: world_manager.map_color_bytes(),
        texture_descriptor: TextureDescriptor {
            label:           None,
            size:            Extent3d {
                width: world.width,
                height: world.height,
                ..default()
            },
            dimension:       TextureDimension::D2,
            format:          TextureFormat::Rgba32Float,
            mip_level_count: 1,
            sample_count:    1,
            usage:           TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        },
        ..default()
    });
    world_manager.map_image_handle_id = Some(map_image_handle.id);

    #[cfg(feature = "planet_view")]
    {
        let world = world_manager.world();
        let planet_image_handle = images.add(Image {
            data: world_manager.planet_color_bytes(),
            texture_descriptor: TextureDescriptor {
                label:           None,
                size:            Extent3d {
                    width: world.width,
                    height: world.height,
                    ..default()
                },
                dimension:       TextureDimension::D2,
                format:          TextureFormat::Rgba32Float,
                mip_level_count: 1,
                sample_count:    1,
                usage:           TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            },
            ..default()
        });
        world_manager.planet_image_handle_id = Some(planet_image_handle.id);

        _ = commands.spawn_bundle(Camera3dBundle {
            camera: Camera {
                is_active: false,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(default(), Vec3::Y),
            projection: OrthographicProjection {
                scale: 0.01,
                ..default()
            }
            .into(),
            ..default()
        });

        let planet_material_handle = materials.add(
            images
                .get_handle(world_manager.planet_image_handle_id.unwrap())
                .into(),
        );
        world_manager.planet_material_handle_id = Some(planet_material_handle.id);

        _ = commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(UVSphere {
                radius: 2.0,
                ..default()
            })),
            material: planet_material_handle,
            transform: Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),
            ..default()
        });
        _ = commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(-20.0, 0.0, 50.0),
            point_light: PointLight {
                intensity: 600000.,
                range: 100.,
                ..default()
            },
            ..default()
        });
    }

    _ = commands
        .spawn_bundle(Camera2dBundle { ..default() })
        .insert(PanCam {
            max_scale: Some(80.0),
            ..default()
        });
    _ = commands.spawn_bundle(SpriteBundle {
        texture: images.get_handle(world_manager.map_image_handle_id.unwrap()),
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
                                    font:      julia_mono_handle.clone(),
                                    font_size: 15.0,
                                    color:     Color::WHITE,
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
                    ToolbarButton::BUTTONS.iter().for_each(|&button_type| {
                        _ = button_box
                            .spawn_bundle(toolbar_button())
                            .with_children(|button| {
                                _ = button.spawn_bundle(toolbar_button_text(
                                    julia_mono_handle.clone(),
                                    button_type,
                                ));
                            })
                            .insert(button_type)
                    });
                });
        });
}

#[cfg(feature = "render")]
const WORLD_SCALE: i32 = 4;
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
                width: (WORLD_SCALE * world.width as i32) as f32,
                height: (WORLD_SCALE * world.height as i32) as f32,
                title: String::from("World-RS"),
                resizable: true,
                ..default()
            })
            .insert_resource(CursorMapPosition::default())
            .add_startup_system(generate_graphics)
            .add_system(handle_toolbar_button)
            .add_system(update_cursor_map_position)
            .add_system(update_info_panel);
        #[cfg(all(feature = "render", feature = "planet_view"))]
        {
            _ = app.add_system(rotate_planet);
        }
    }
    #[cfg(not(feature = "render"))]
    {
        _ = manager.new_world()?
    }

    _ = app.insert_resource(LogSettings {
        #[cfg(feature = "debug")]
        level: Level::DEBUG,
        #[cfg(not(feature = "debug"))]
        level: Level::WARN,
        ..default()
    });

    app.add_plugins(WorldPlugins).insert_resource(manager).run();

    Ok(())
}
