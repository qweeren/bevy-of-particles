use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::config::UI_PANEL_DEFAULT_WIDTH;
use crate::plugins::input::resources::{BrushSize, SelectedMaterial};
use crate::registry::MATERIAL_NAMES;
use bevy::window::PrimaryWindow;
use crate::utils::grid_utils::get_grid_pos;

/// UI system that displays a sidebar with material selection buttons and a brush preview.
pub fn ui_system(
    mut egui_context: EguiContexts,
    mut selected_material: ResMut<SelectedMaterial>,
    mut brush_size: ResMut<BrushSize>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_q.single();

    // Draw the brush preview circle
    if let Some(cursor_pos) = window.cursor_position() {
        let radius = brush_size.0 as f32 * 2.0; // Multiply by 2 for better visibility
        let painter = egui_context.ctx_mut().layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("brush_preview")));
        painter.circle_stroke(
            egui::pos2(cursor_pos.x, cursor_pos.y),
            radius,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
    }

    // Original sidebar UI
    egui::SidePanel::right("right_panel")
        .resizable(false)
        .default_width(UI_PANEL_DEFAULT_WIDTH)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Select Material");

            for (id, name) in MATERIAL_NAMES.iter().enumerate() {
                if ui.selectable_label(selected_material.0 == id as u8, *name).clicked() {
                    selected_material.0 = id as u8;
                }
            }

            ui.separator();
            ui.label("Current Material:");
            let current_name = if (selected_material.0 as usize) < MATERIAL_NAMES.len() {
                MATERIAL_NAMES[selected_material.0 as usize]
            } else {
                "Unknown"
            };
            ui.label(current_name);
            ui.separator();
            ui.add(egui::Slider::new(&mut brush_size.0, 1..=30).text("Brush Size"));
        });
}
