pub(crate) struct WorldPlugins;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::CorePlugin,
    diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    time::TimePlugin,
};

impl PluginGroup for WorldPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        _ = group.add(LogPlugin).add(CorePlugin).add(TimePlugin);

        #[cfg(feature = "render")]
        {
            use {
                crate::plugins::panning_plugin::PanningPlugin,
                bevy::{
                    asset::AssetPlugin,
                    core_pipeline::CorePipelinePlugin,
                    hierarchy::HierarchyPlugin,
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

            _ = group
                .add(TransformPlugin)
                // hierarchy
                .add(InputPlugin)
                .add(WindowPlugin)
                .add(AssetPlugin)
                .add(HierarchyPlugin)
                .add(WinitPlugin)
                .add(RenderPlugin)
                .add(CorePipelinePlugin)
                .add(SpritePlugin)
                .add(TextPlugin)
                .add(UiPlugin)
                .add(PanningPlugin)
                .add(EguiPlugin);
        }
        #[cfg(not(feature = "render"))]
        {
            use bevy::app::ScheduleRunnerPlugin;
            _ = group.add(ScheduleRunnerPlugin);
        }

        _ = group.add(DiagnosticsPlugin);
        #[cfg(all(feature = "logging"))]
        {
            use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
            _ = group.add(FrameTimeDiagnosticsPlugin);
        }
        _ = group.add(LogDiagnosticsPlugin::default());
    }
}
