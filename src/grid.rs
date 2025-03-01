use bevy::prelude::*;
use crate::config::{GRID_WIDTH, GRID_HEIGHT};
use crate::materials::Material;
use crate::materials::types::Particle;

/// The simulation grid, storing materials in a 1D vector
#[derive(Clone, Resource)]
pub struct Grid {
    pub particles: Vec<Particle>,
    pub velocities: Vec<f32>,
}

impl Grid {
    /// Creates a new grid, initialized with empty cells (0)
    pub fn new() -> Self {
        Grid {
            particles: vec![Particle::new(Material::Empty); GRID_WIDTH * GRID_HEIGHT],
            velocities: vec![0.0; GRID_WIDTH * GRID_HEIGHT],
        }
    }

    /// Gets the material at (x, y)
    pub fn get(&self, x: usize, y: usize) -> Particle {
        self.particles[y * GRID_WIDTH + x]
    }

    /// Sets the material at (x, y)
    pub fn set(&mut self, x: usize, y: usize, material: Material) {
        self.particles[y * GRID_WIDTH + x] = Particle::new(material);
    }

    /// Checks if (x, y) is within the grid bounds
    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < GRID_WIDTH as isize && y >= 0 && y < GRID_HEIGHT as isize
    }

    /// Swaps materials between two cells
    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let idx1 = y1 * GRID_WIDTH + x1;
        let idx2 = y2 * GRID_WIDTH + x2;
        self.particles.swap(idx1, idx2);
    }

    /// Moves material from (x1, y1) to (x2, y2) if the target is empty (0)
    pub fn move_to(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if self.get(x2, y2).material_type == Material::Empty as u8 {
            self.swap(x1, y1, x2, y2);
        }
    }

    pub fn get_velocity(&self, x: usize, y: usize) -> f32 {
        self.velocities[y * GRID_WIDTH + x]
    }

    pub fn set_velocity(&mut self, x: usize, y: usize, velocity: f32) {
        self.velocities[y * GRID_WIDTH + x] = velocity;
    }

    pub fn move_to_with_velocity(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if self.get(x2, y2).material_type == Material::Empty as u8 {
            let particle = self.get(x1, y1);
            
            self.set(x2, y2, Material::from_id(particle.material_type));
            self.set(x1, y1, Material::Empty);
            
            self.set_velocity(x2, y2, self.get_velocity(x1, y1));
            self.set_velocity(x1, y1, 0.0);
        }
    }
}
