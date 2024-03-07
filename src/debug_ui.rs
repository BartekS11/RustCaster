use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui, quick::WorldInspectorPlugin};

impl Plugin for DebugUI {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_systems(Update, bevy::window::close_on_esc)
            .add_systems(Update, inspector_ui)
            .add_systems(Update, display_debug_stats);
    }
}

pub struct DebugUI;

fn inspector_ui(world: &mut World, mut disabled: Local<bool>) {
    let space_pressed = world
        .resource::<ButtonInput<KeyCode>>()
        .just_pressed(KeyCode::KeyC);
    match space_pressed {
        true => {
            *disabled = !*disabled;
            info!("sdadasd");
        }
        false => {}
    }
}

fn display_debug_stats(mut egui: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    egui::Window::new("performance stuff").show(egui.ctx_mut(), |ui| {
        ui.label(format!(
            "Avg. FPS: {:.02}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .smoothed()
                .unwrap_or_default()
        ));
        ui.label(format!(
            "Total Entity count: {}",
            diagnostics
                .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .unwrap()
                .smoothed()
                .unwrap_or_default()
        ));
    });
}
