// Grid Configuration
pub const GRID_WIDTH: usize = 100;
pub const GRID_HEIGHT: usize = 100;
pub const CELL_SIZE: f32 = 10.0;

// UI Configuration
pub const UI_SIDEBAR_WIDTH: f32 = 230.0;
pub const UI_PANEL_DEFAULT_WIDTH: f32 = 100.0;

// Window Configuration
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}

impl WindowConfig {
    pub fn new() -> Self {
        // Calculate window dimensions based on grid and UI sizes
        let width = (GRID_WIDTH as f32 * CELL_SIZE) + UI_SIDEBAR_WIDTH;
        let height = GRID_HEIGHT as f32 * CELL_SIZE;
        
        Self { width, height }
    }
}

// Camera Configuration
pub const CAMERA_OFFSET_X: f32 = 115.0; // If this is still needed, document why

// Derived Constants
pub const GRID_PIXEL_WIDTH: f32 = GRID_WIDTH as f32 * CELL_SIZE;
pub const GRID_PIXEL_HEIGHT: f32 = GRID_HEIGHT as f32 * CELL_SIZE;