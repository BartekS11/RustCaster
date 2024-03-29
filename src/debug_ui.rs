use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::*,
};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui, quick::WorldInspectorPlugin};

pub struct DebugUI;

impl Plugin for DebugUI {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_systems(Update, close_on_esc)
            .add_systems(Update, display_debug_stats);
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
