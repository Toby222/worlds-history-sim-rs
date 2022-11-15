#![cfg_attr(not(feature = "logging"), windows_subsystem = "windows")]

use {
    crate::resources::GenerateWorldTask,
    futures_lite::future::{block_on, poll_once},
    resources::GenerateWorldProgressChannel,
};

pub mod components;
#[cfg(feature = "render")]
pub mod gui;
pub mod macros;
#[cfg(feature = "render")]
pub mod planet_renderer;
pub mod plugins;
pub mod resources;

use {bevy::prelude::*, planet::WorldManager, plugins::WorldPlugins};
#[cfg(feature = "render")]
use {
    bevy::render::camera::RenderTarget,
    bevy_egui::{
        egui::{FontData, FontDefinitions, FontFamily, ProgressBar},
        EguiContext,
    },
    gui::{render_windows, widget, widgets::ToolbarWidget, window::open_window, windows::TileInfo},
    planet_renderer::{WorldRenderSettings, WorldRenderer},
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
    mut cursor_map_position: ResMut<CursorMapPosition>,
    transform: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    windows: Res<Windows>,
    world_manager: Res<WorldManager>,
) {
    let Some(world) = world_manager.get_world() else {
        return
    };
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

        cursor_map_position.x = world.width as i32 / 2 + f32::ceil(world_position.x) as i32 - 1;
        cursor_map_position.y = world.height as i32 / 2 + f32::ceil(world_position.y) as i32 - 1;
    }
}

fn handle_generate_world_task(
    mut generate_world_task: ResMut<GenerateWorldTask>,
    mut world_manager: ResMut<WorldManager>,
    #[cfg(feature = "render")] mut should_redraw: ResMut<ShouldRedraw>,
    #[cfg(feature = "render")] mut egui_ctx: ResMut<'_, EguiContext>,
    #[cfg(feature = "render")] progress_channel: Res<'_, GenerateWorldProgressChannel>,
    #[cfg(feature = "render")] mut progress: Local<(f32, String)>,
) {
    if let Some(task) = &mut generate_world_task.0 {
        if task.is_finished() {
            debug!("Done generating world");
            if let Some(result) = block_on(poll_once(task)) {
                match result {
                    Ok(world) => {
                        world_manager.set_world(world);
                        #[cfg(feature = "render")]
                        {
                            should_redraw.0 = true;
                            #[cfg(feature = "logging")]
                            debug!("Requesting map redraw");
                        }
                    },
                    Err(err) => error!("{err:#?}"),
                }
            }
            generate_world_task.0 = None;
            #[cfg(feature = "render")]
            {
                *progress = (0.0, String::from("Generating world..."));
            }
        } else {
            debug!("Still generating world");

            #[cfg(feature = "render")]
            {
                if let Ok(new_progress) = progress_channel.receiver().try_recv() {
                    *progress = new_progress;
                }
                _ = bevy_egui::egui::TopBottomPanel::bottom("Generating World ProgressBar")
                    .default_height(8.0)
                    .show(egui_ctx.ctx_mut(), |ui| {
                        ui.add(ProgressBar::new(progress.0).text(progress.1.as_str()));
                    });
            }
        }
    }
}

#[cfg(feature = "render")]
fn generate_graphics(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    egui_context: ResMut<EguiContext>,
    render_settings: ResMut<WorldRenderSettings>,
) {
    // Add Julia-Mono font to egui
    {
        let egui_context = egui_context.into_inner();
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
        // Make all text 33% bigger because I have bad eyes (and the font is small)
        for style in style.text_styles.iter_mut() {
            style.1.size *= 16.0 / 12.0;
        }
        ctx.set_style(style);
        // #[cfg(feature = "logging")]
        // debug!("Fonts: {:#?}", &ctx.style().text_styles);
    }

    // Set up 2D map mode
    {
        use bevy::render::render_resource::{
            TextureDescriptor,
            TextureDimension,
            TextureFormat,
            TextureUsages,
        };
        let images = images.into_inner();
        let mut render_settings = render_settings.into_inner();
        let map_image_handle = images.add(Image {
            data: vec![0; 16],
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
        render_settings.map_image_handle_id = Some(map_image_handle.id());
        _ = commands.spawn(Camera2dBundle::default());

        // TODO: Switch to egui
        _ = commands.spawn(SpriteBundle {
            texture: images.get_handle(render_settings.map_image_handle_id.unwrap()),
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
    world.resource_scope(|world, mut ctx: Mut<EguiContext>| {
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
    render_settings: Res<WorldRenderSettings>,
    mut images: ResMut<Assets<Image>>,
    mut map_sprite: Query<&mut Sprite>,
) {
    let Some(world) = world_manager.get_world() else {
        #[cfg(feature = "logging")]
        if should_redraw.0 {
            debug!("Couldn't redraw map despite wanting to, because world isn't generated");
        }
        return;
    };
    assert!(world.width > 0);

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
        #[cfg(feature = "logging")]
        debug!("Resizing image to {}x{}", world.width, world.height);
        map_image.resize(bevy::render::render_resource::Extent3d {
            width: world.width,
            height: world.height,
            ..default()
        });
        map_image.data = world_manager.map_color_bytes(render_settings);
        map_sprite.single_mut().custom_size = Some(Vec2 {
            x: (world.width * WORLD_SCALE as u32) as f32,
            y: (world.height * WORLD_SCALE as u32) as f32,
        });

        should_redraw.0 = false;
    }
}

#[cfg(feature = "render")]
const WORLD_SCALE: i32 = 4;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    #[cfg(feature = "render")]
    {
        use bevy::winit::{UpdateMode, WinitSettings};

        _ = app
            .insert_resource(WinitSettings {
                focused_mode: UpdateMode::Continuous,
                unfocused_mode: UpdateMode::ReactiveLowPower {
                    max_wait: std::time::Duration::from_secs(10),
                },
                ..default()
            })
            .insert_resource(CursorMapPosition::default())
            .insert_resource(OpenedWindows::default())
            .insert_resource(WorldRenderSettings::default())
            .insert_resource(ShouldRedraw::default())
            .add_startup_system(generate_graphics)
            .add_system(update_gui)
            .add_system(update_cursor_map_position)
            .add_system(open_tile_info)
            .add_system(redraw_map);

        app.add_plugins(WorldPlugins);
    }
    #[cfg(not(feature = "render"))]
    {
        app.add_plugins(WorldPlugins);
    }

    app.insert_resource(WorldManager::new())
        .insert_resource(GenerateWorldProgressChannel::new())
        .insert_resource(GenerateWorldTask(None))
        .add_system(handle_generate_world_task)
        .run();

    Ok(())
}
