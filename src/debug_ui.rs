use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::*,
};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui, quick::WorldInspectorPlugin};

use crate::common_utils::CrosshairPlugin;

pub struct DebugUI<S: States> {
    pub state: S,
}

impl<S: States> Plugin for DebugUI<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::default().run_if(in_state(self.state.clone())))
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_plugins(CrosshairPlugin)
            .add_systems(
                Update,
                display_debug_stats.run_if(in_state(self.state.clone()))
            )
            .add_systems(Update, close_on_esc);
    }
}

fn display_debug_stats(mut egui: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    egui::Window::new("Performance stuff").show(egui.ctx_mut(), |ui| {
        ui.label(format!(
            "Avg. FPS: {:.02}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .smoothed()
                .unwrap_or_default()
        ));
        ui.label(format!(
            "Total entity count: {}",
            diagnostics
                .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .unwrap()
                .smoothed()
                .unwrap_or_default()
        ));
    });
}
