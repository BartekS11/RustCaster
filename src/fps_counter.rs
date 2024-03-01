use bevy::{prelude::*, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin}};
use bevy_inspector_egui::{egui, bevy_egui::EguiContexts};

impl Plugin for FpsCounter {
    fn build(&self, app: &mut App) {
            app.add_systems(Update, inspector_ui)
            // .add_systems(Update, display_debug_stats);
    }
}

pub struct FpsCounter;

fn inspector_ui(world: &mut World, mut disabled: Local<bool>) {
    let space_pressed = world
        .resource::<Input<KeyCode>>()
        .just_pressed(KeyCode::C);
    if space_pressed {
        *disabled = !*disabled;
    }
    if *disabled {
        return;
    }
}

fn display_debug_stats(mut egui: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    egui::Window::new("performance stuff").show(egui.ctx_mut(), |ui| {
        ui.label(format!(
            "Avg. FPS: {:.02}",
            diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .average()
                .unwrap_or_default()
        ));
        ui.label(format!(
            "Total Entity count: {}",
            diagnostics
                .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .unwrap()
                .average()
                .unwrap_or_default()
        ));
    });
}