use bevy::{
    a11y::AccessibilityPlugin,
    app::ScheduleRunnerPlugin,
    core_pipeline::CorePipelinePlugin,
    input::InputPlugin,
    log::LogPlugin,
    pbr::PbrPlugin,
    prelude::*,
    render::{render_asset::RenderAssetPlugin, RenderPlugin},
    time::TimePlugin,
    winit::WinitPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            WindowPlugin::default(),
            WinitPlugin,
            InputPlugin,
            AccessibilityPlugin,
            // For tracy
            TimePlugin::default(),
            TaskPoolPlugin::default(),
            AssetPlugin::default(),
            RenderPlugin::default(),
            ImagePlugin::default(),
            LogPlugin::default(),
            ScheduleRunnerPlugin::default(),
            CorePipelinePlugin::default(),
            FrameCountPlugin::default(),
            PbrPlugin::default(),
        ))
        .run();
}
