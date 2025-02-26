use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{input::input::{BrushSize, SelectedMaterial}, registry::MATERIAL_NAMES};

/// UI system that displays a sidebar with material selection buttons.
pub fn ui_system(
    mut egui_context: EguiContexts,
    mut selected_material: ResMut<SelectedMaterial>,
    mut brush_size: ResMut<BrushSize>
) {
    egui::SidePanel::right("right_panel")
        .resizable(false)
        .default_width(100.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Select Material");

            // Dynamically create buttons for each material
            for (id, name) in MATERIAL_NAMES.iter().enumerate() {
                if ui.selectable_label(selected_material.0 == id as u8, *name).clicked() {
                    selected_material.0 = id as u8;
                }
            }

            ui.separator();
            ui.label("Current Material:");
            // Display the name of the selected material, or "Unknown" if invalid
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
