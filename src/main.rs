#![cfg_attr(not(feature = "logging"), windows_subsystem = "windows")]

pub(crate) mod components;
#[cfg(feature = "render")]
pub(crate) mod gui;
pub(crate) mod macros;
pub(crate) mod plugins;
#[cfg(feature = "render")]
pub(crate) mod resources;

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
        input::{keyboard::KeyCode, Input},
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
    gui::{render_windows, widget, widgets::ToolbarWidget, window::open_window, windows::TileInfo},
    planet::WorldRenderSettings,
    resources::{CursorMapPosition, OpenedWindows, ShouldRedraw},
};
#[cfg(all(feature = "render", feature = "logging"))]
use {
    bevy::{
        diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
        log::debug,
    },
    bevy_egui::egui::Frame,
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

#[cfg(feature = "render")]
fn generate_graphics(
    mut commands: Commands<'_, '_>,
    world_manager: ResMut<'_, WorldManager>,
    mut images: ResMut<'_, Assets<Image>>,
    mut egui_context: ResMut<'_, EguiContext>,
    mut render_settings: ResMut<'_, WorldRenderSettings>,
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

        let mut style = (*ctx.style()).clone();
        for style in style.text_styles.iter_mut() {
            style.1.size *= 16.0 / 12.0;
        }
        ctx.set_style(style);
        debug!("Fonts: {:#?}", &ctx.style().text_styles);
    }

    let world = world_manager.world();
    let custom_sprite_size = Vec2 {
        x: (WORLD_SCALE * world.width as i32) as f32,
        y: (WORLD_SCALE * world.height as i32) as f32,
    };
    // Set up 2D map mode
    {
        let map_image_handle = images.add(Image {
            data: vec![],
            texture_descriptor: TextureDescriptor {
                label:           None,
                size:            default(),
                dimension:       TextureDimension::D2,
                format:          TextureFormat::Rgba32Float,
                mip_level_count: 1,
                sample_count:    1,
                usage:           TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            },
            ..default()
        });
        render_settings.map_image_handle_id = Some(map_image_handle.id);
        _ = commands
            .spawn_bundle(Camera2dBundle::default())
            .insert(Pan2d::new());

        // TODO: Switch to egui
        _ = commands.spawn_bundle(SpriteBundle {
            texture: images.get_handle(map_image_handle.id),
            sprite: Sprite {
                custom_size: Some(custom_sprite_size),
                ..default()
            },
            ..default()
        });
    }
}

#[cfg(feature = "render")]
fn open_tile_info(mut windows: ResMut<OpenedWindows>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::I) {
        open_window::<TileInfo>(&mut windows);
    }
}

#[cfg(feature = "render")]
fn update_gui(world: &mut World) {
    world.resource_scope(|world, mut ctx: Mut<'_, EguiContext>| {
        let ctx = ctx.ctx_mut();
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

        render_windows(world, ctx);
    });
}

#[cfg(feature = "render")]
fn redraw_map(
    mut should_redraw: ResMut<ShouldRedraw>,
    world_manager: Res<WorldManager>,
    render_settings: Res<'_, WorldRenderSettings>,
    mut images: ResMut<Assets<Image>>,
) {
    if should_redraw.0 {
        let world_manager: &WorldManager = &world_manager;
        let render_settings: &WorldRenderSettings = &render_settings;
        let images: &mut Assets<Image> = &mut images;
        #[cfg(feature = "logging")]
        debug!("refreshing world texture");
        let map_image_handle = images.get_handle(
            render_settings
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
        map_image.data = world_manager.map_color_bytes(render_settings);

        should_redraw.0 = false;
    }
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
            .insert_resource(OpenedWindows::default())
            .insert_resource(WorldRenderSettings::default())
            .insert_resource(ShouldRedraw::default())
            .add_startup_system(generate_graphics)
            .add_system(update_gui.exclusive_system())
            .add_system(update_cursor_map_position)
            .add_system(open_tile_info)
            .add_system(redraw_map);
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
