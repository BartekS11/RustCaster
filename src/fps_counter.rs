use bevy::{diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, input::keyboard::KeyboardInput, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui, quick::WorldInspectorPlugin};

impl Plugin for FpsCounter {
    fn build(&self, app: &mut App) {
        
            app.add_plugins(WorldInspectorPlugin::default())
            .add_systems(Update, bevy::window::close_on_esc)
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_systems(Update, inspector_ui)
            .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_systems(Update, display_debug_stats);
    }
}

pub struct FpsCounter;

fn inspector_ui(world: &mut World, mut disabled: Local<bool>) {
    let space_pressed = world
        .resource::<ButtonInput<KeyCode>>()
        .just_pressed(KeyCode::KeyC);
    match space_pressed {
        true => {
            *disabled = !*disabled;
            info!("sdadasd");
        }
        false => return,
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