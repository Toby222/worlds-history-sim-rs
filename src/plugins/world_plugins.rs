pub(crate) struct WorldPlugins;

#[cfg(not(feature = "render"))]
use bevy::app::ScheduleRunnerPlugin;
#[cfg(all(feature = "logging"))]
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::CorePlugin,
    log::{Level, LogPlugin},
    prelude::*,
    time::TimePlugin,
};
#[cfg(feature = "render")]
use {
    bevy::{
        asset::AssetPlugin,
        core_pipeline::CorePipelinePlugin,
        input::InputPlugin,
        render::RenderPlugin,
        sprite::SpritePlugin,
        text::TextPlugin,
        transform::TransformPlugin,
        ui::UiPlugin,
        window::WindowPlugin,
        winit::WinitPlugin,
    },
    bevy_egui::EguiPlugin,
};

impl PluginGroup for WorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group_builder = PluginGroupBuilder::start::<Self>()
            .add(LogPlugin {
                #[cfg(feature = "logging")]
                level: Level::DEBUG,
                #[cfg(not(feature = "logging"))]
                level: Level::WARN,
                ..default()
            })
            .add(CorePlugin::default()) // sets compute pool config
            .add(TimePlugin);

        #[cfg(feature = "render")]
        {
            group_builder = group_builder
                .add(TransformPlugin)
                .add(InputPlugin)
                .add(WindowPlugin::default())
                .add(AssetPlugin::default())
                .add(RenderPlugin)
                .add(ImagePlugin::default_nearest())
                .add(WinitPlugin)
                .add(CorePipelinePlugin)
                .add(SpritePlugin)
                .add(TextPlugin)
                .add(UiPlugin)
                .add(EguiPlugin);
        }
        #[cfg(not(feature = "render"))]
        {
            group_builder = group_builder.add(ScheduleRunnerPlugin);
        }

        #[cfg(feature = "logging")]
        {
            group_builder = group_builder
                .add(DiagnosticsPlugin)
                .add(FrameTimeDiagnosticsPlugin)
                .add(LogDiagnosticsPlugin::default());
        }

        group_builder
    }
}
