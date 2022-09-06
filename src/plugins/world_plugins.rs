pub(crate) struct WorldPlugins;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::CorePlugin,
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    time::TimePlugin,
};

impl PluginGroup for WorldPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        _ = group
            .add(LogPlugin::default())
            .add(CorePlugin::default())
            .add(TimePlugin::default());

        #[cfg(feature = "render")]
        {
            use bevy::{
                asset::AssetPlugin, core_pipeline::CorePipelinePlugin, hierarchy::HierarchyPlugin,
                input::InputPlugin, pbr::PbrPlugin, render::RenderPlugin, sprite::SpritePlugin,
                text::TextPlugin, transform::TransformPlugin, ui::UiPlugin, window::WindowPlugin,
                winit::WinitPlugin,
            };
            use bevy_pancam::PanCamPlugin;

            _ = group
                .add(TransformPlugin::default())
                // hierarchy
                .add(InputPlugin::default())
                .add(WindowPlugin::default())
                .add(AssetPlugin::default())
                .add(HierarchyPlugin::default())
                .add(WinitPlugin::default())
                .add(RenderPlugin::default())
                .add(CorePipelinePlugin::default())
                .add(SpritePlugin::default())
                .add(TextPlugin::default())
                .add(UiPlugin::default())
                .add(PbrPlugin::default())
                .add(PanCamPlugin::default());
        }
        #[cfg(not(feature = "render"))]
        {
            use bevy::app::ScheduleRunnerPlugin;
            _ = group.add(ScheduleRunnerPlugin::default());
        }

        _ = group
            .add(DiagnosticsPlugin::default())
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(LogDiagnosticsPlugin::default());
    }
}
