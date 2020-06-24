use crate::{GRID_SLOT_AMOUNT, GRID_SLOT_SIZE};
use crate::particle_behaviour::*;

use sdl2::pixels::Color;
use rand::prelude::*;

pub struct ParticlePlane {
    pub grid: [[Option<Particle>; GRID_SLOT_AMOUNT.0 as usize]; GRID_SLOT_AMOUNT.1 as usize],
    pub rng: ThreadRng,
}

impl ParticlePlane {
    pub fn new() -> Self {
        ParticlePlane { grid: [[None; GRID_SLOT_AMOUNT.0 as usize]; GRID_SLOT_AMOUNT.1 as usize], rng: thread_rng() }
    }

    pub fn update(&mut self) {
        for x in 0..GRID_SLOT_AMOUNT.0 as usize {
            for y in 0..GRID_SLOT_AMOUNT.1 as usize {
                if let Some(particle) = &self.grid[x][y] {
                    if particle.updateable {
                        match particle.ptype {
                            ParticleType::Sand => move_powder(self, x, y),
                            ParticleType::Water => move_powder(self, x, y),
                        }
                    }
                }
            }
        }
    }

    pub fn reset_updateable(&mut self) {
        for x in 0..GRID_SLOT_AMOUNT.0 as usize {
            for y in 0..GRID_SLOT_AMOUNT.1 as usize {
                if self.grid[x][y].is_some() {
                    self.grid[x][y].as_mut().unwrap().updateable = true;
                }
            }
        }
    }

    pub fn draw<T: sdl2::render::RenderTarget>(&self, canvas: &mut sdl2::render::Canvas<T>) {
        for x in 0..GRID_SLOT_AMOUNT.0 {
            for y in 0..GRID_SLOT_AMOUNT.1 {
                if self.grid[x as usize][y as usize].is_some() {
                    let mut color: (u8, u8, u8) = match self.grid[x as usize][y as usize].unwrap().ptype {
                        ParticleType::Sand => (150, 130, 120),
                        ParticleType::Water => (0, 0, 230),
                    };
                    canvas.set_draw_color(Color::RGB(color.0 + self.grid[x as usize][y as usize].unwrap().color_offset.0 as u8,
                                                     color.1 + self.grid[x as usize][y as usize].unwrap().color_offset.1 as u8,
                                                     color.2 + self.grid[x as usize][y as usize].unwrap().color_offset.2 as u8));
                    canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * GRID_SLOT_SIZE.0, y as i32 * GRID_SLOT_SIZE.1, GRID_SLOT_SIZE.0 as u32, GRID_SLOT_SIZE.1 as u32));
                }
                
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParticleType {
    Sand,
    Water,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParticleState {
    SuperSolid,
    Solid,
    Liquid,
    Gas,
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub ptype: ParticleType,
    pub state: ParticleState,
    pub temp: u16,
    pub density: u16,
    pub xd1: i8,
    pub xd2: i8,
    pub color_offset: (i8, i8, i8),
    pub updateable: bool,
}

impl Particle {
    pub fn new(ptype: ParticleType) -> Self {
        Self {
            ptype,
            temp: match ptype {
                ParticleType::Sand => 0,
                ParticleType::Water => 1000,
            },
            state: match ptype {
                ParticleType::Sand => ParticleState::Solid,
                ParticleType::Water => ParticleState::Liquid,
            },
            density: match ptype {
                ParticleType::Sand => 200,
                ParticleType::Water => 100,
            },
            xd1: match ptype {
                ParticleType::Sand => 0,
                ParticleType::Water => 0,
            },
            xd2: match ptype {
                ParticleType::Sand => 0,
                ParticleType::Water => 0,
            },
            color_offset: (0, 0, 0), //TODO
            updateable: true,
        }
    }
}