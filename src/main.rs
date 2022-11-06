pub(crate) mod components;
#[cfg(feature = "render")]
pub(crate) mod gui;
pub(crate) mod macros;
pub(crate) mod plugins;
pub(crate) mod resources;

#[cfg(all(feature = "render", feature = "logging"))]
use {
    bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    bevy_egui::egui::Frame,
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
#[cfg(feature = "render")]
use {
    bevy::{
        asset::Assets,
        core_pipeline::core_2d::{Camera2d, Camera2dBundle},
        ecs::{
            change_detection::{Mut, ResMut},
            query::With,
            system::{Commands, IntoExclusiveSystem, Query, Res},
            world::World,
        },
        prelude::Vec2,
        render::{
            camera::{Camera, RenderTarget},
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
        transform::components::GlobalTransform,
        window::{WindowDescriptor, Windows},
        winit::WinitSettings,
    },
    bevy_egui::{
        egui::{FontData, FontDefinitions, FontFamily},
        EguiContext,
    },
    components::panning::Pan2d,
    gui::{
        widget,
        widgets::{InfoPanel, ToolbarWidget},
    },
    resources::CursorMapPosition,
};
#[cfg(all(feature = "render", feature = "globe_view"))]
use {
    bevy::{
        asset::Handle,
        core_pipeline::core_3d::Camera3dBundle,
        pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
        prelude::{Quat, Vec3},
        render::camera::OrthographicProjection,
        render::mesh::{shape::UVSphere, Mesh},
        time::Time,
        transform::components::Transform,
    },
    std::f32::consts::FRAC_PI_2,
};

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

#[cfg(all(feature = "render", feature = "globe_view"))]
const GLOBE_ROTATIONS_PER_SECOND: f32 = std::f32::consts::TAU / 15.0;
#[cfg(all(feature = "render", feature = "globe_view"))]
fn rotate_globe(
    mut globe_transform: Query<'_, '_, &mut Transform, With<Handle<Mesh>>>,
    time: Res<Time>,
) {
    globe_transform
        .single_mut()
        .rotate_y(GLOBE_ROTATIONS_PER_SECOND * time.delta_seconds());
}

#[cfg(feature = "render")]
fn generate_graphics(
    mut commands: Commands<'_, '_>,
    mut world_manager: ResMut<'_, WorldManager>,
    mut images: ResMut<'_, Assets<Image>>,
    mut egui_context: ResMut<'_, EguiContext>,
    #[cfg(feature = "globe_view")] mut materials: ResMut<'_, Assets<StandardMaterial>>,
    #[cfg(feature = "globe_view")] mut meshes: ResMut<'_, Assets<Mesh>>,
) {
    // Add Julia-Mono font to egui
    {
        let ctx = egui_context.ctx_mut();
        let mut fonts = FontDefinitions::default();
        const FONT_NAME: &str = "Julia-Mono";
        _ = fonts.font_data.insert(
            FONT_NAME.to_owned(),
            FontData::from_static(include_bytes!("../assets/JuliaMono.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .expect("Failed to get 'Monospace' FontFamily")
            .insert(0, FONT_NAME.to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .expect("Failed to get 'Proportional' FontFamily")
            .push(FONT_NAME.to_owned());
        ctx.set_fonts(fonts);
    }

    let world = world_manager.world();
    let custom_sprite_size = Vec2 {
        x: (WORLD_SCALE * world.width as i32) as f32,
        y: (WORLD_SCALE * world.height as i32) as f32,
    };
    // Set up 2D map mode
    {
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
        _ = commands
            .spawn_bundle(Camera2dBundle::default())
            .insert(Pan2d::new());

        // TODO: Switch to egui
        _ = commands.spawn_bundle(SpriteBundle {
            texture: images.get_handle(world_manager.map_image_handle_id.unwrap()),
            sprite: Sprite {
                custom_size: Some(custom_sprite_size),
                ..default()
            },
            ..default()
        });
    }

    #[cfg(feature = "globe_view")]
    {
        let world = world_manager.world();
        let globe_image_handle = images.add(Image {
            data: world_manager.globe_color_bytes(),
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
        world_manager.globe_image_handle_id = Some(globe_image_handle.id);

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

        let globe_material_handle = materials.add(
            images
                .get_handle(world_manager.globe_image_handle_id.unwrap())
                .into(),
        );
        world_manager.globe_material_handle_id = Some(globe_material_handle.id);

        // TODO: Globe texture is mirrored east-to-west.
        _ = commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(UVSphere {
                radius: 2.0,
                ..default()
            })),
            material: globe_material_handle,
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
}

#[cfg(feature = "render")]
fn update_gui(world: &mut World) {
    world.resource_scope(|world, mut ctx: Mut<'_, EguiContext>| {
        let ctx = ctx.ctx_mut();
        _ = bevy_egui::egui::Window::new("Tile Info")
            .resizable(false)
            .show(ctx, |ui| {
                widget::<InfoPanel<'_, '_>>(world, ui, "Tile Info Panel".into());
            });

        #[cfg(feature = "logging")]
        {
            bevy_egui::egui::CentralPanel::default()
                .frame(Frame::none())
                .show(ctx, |ui| {
                    _ = ui.label(format!(
                        "{:.0}",
                        match world
                            .resource::<Diagnostics>()
                            .get_measurement(FrameTimeDiagnosticsPlugin::FPS)
                        {
                            None => f64::NAN,
                            Some(fps) => fps.value,
                        }
                    ));
                });
        }

        _ = bevy_egui::egui::TopBottomPanel::bottom("Toolbar")
            .resizable(false)
            .default_height(30.0)
            .show(ctx, |ui| {
                widget::<ToolbarWidget<'_, '_>>(world, ui, "Toolbar".into());
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
            .add_system(update_gui.exclusive_system())
            .add_system(update_cursor_map_position);
        #[cfg(all(feature = "render", feature = "globe_view"))]
        {
            _ = app.add_system(rotate_globe);
        }
    }
    #[cfg(not(feature = "render"))]
    {
        _ = manager.new_world()?
    }

    _ = app.insert_resource(LogSettings {
        #[cfg(feature = "logging")]
        level: Level::DEBUG,
        #[cfg(not(feature = "logging"))]
        level: Level::WARN,
        ..default()
    });

    app.add_plugins(WorldPlugins).insert_resource(manager).run();

    Ok(())
}
