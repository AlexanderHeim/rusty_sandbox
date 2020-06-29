use crate::{GRID_SLOT_AMOUNT, GRID_SLOT_SIZE};
use crate::particle_behaviour::*;
use crate::finals::*;

use sdl2::pixels::Color;
use rand::prelude::*;

pub struct ParticlePlane {
    pub grid: Box<[[Option<Particle>; GRID_SLOT_AMOUNT.0 as usize]; GRID_SLOT_AMOUNT.1 as usize]>,
    pub rng: ThreadRng,
}

impl ParticlePlane {
    pub fn new() -> Self {
        ParticlePlane { grid: Box::new([[None; GRID_SLOT_AMOUNT.0 as usize]; GRID_SLOT_AMOUNT.1 as usize]), rng: thread_rng() }
    }

    pub fn update(&mut self) {
        for x in 0..GRID_SLOT_AMOUNT.0 as usize {
            for y in 0..GRID_SLOT_AMOUNT.1 as usize {
                if self.grid[x][y].is_some() {
                    update_specific_particle(self, x, y);
                    transfer_heat(self, x, y);
                }
                if let Some(particle) = &self.grid[x][y] {
                    if particle.updateable {
                        match particle.ptype {
                            ParticleType::Sand => move_powder(self, x, y),
                            ParticleType::MoltenGlass => move_liquid(self, x, y),
                            ParticleType::Glass => move_solid(self, x, y),
                            ParticleType::Water => move_liquid(self, x, y),
                            ParticleType::Ice => move_solid(self, x, y),
                            ParticleType::Lava => move_liquid(self, x, y),
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
                    let color: (u8, u8, u8) = match self.grid[x as usize][y as usize].unwrap().ptype {
                        ParticleType::Sand => SAND_COLOR,
                        ParticleType::MoltenGlass => MOLTENGLASS_COLOR,
                        ParticleType::Glass => GLASS_COLOR,
                        ParticleType::Water => WATER_COLOR,
                        ParticleType::Ice => ICE_COLOR,
                        ParticleType::Lava => (230, 10, 10),
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
    MoltenGlass,
    Glass,
    Water,
    Ice,
    Lava,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParticleState {
    SuperSolid,
    Solid,
    Powder,
    Liquid,
    Gas,
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub ptype: ParticleType,
    pub state: ParticleState,
    pub temp: u64,
    pub density: u16,
    pub conductivity: u8,
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
                ParticleType::Sand => SAND_STANDARD_TEMP,
                ParticleType::MoltenGlass => MOLTENGLASS_STANDARD_TEMP,
                ParticleType::Glass => GLASS_STANDARD_TEMP,
                ParticleType::Water => WATER_STANDARD_TEMP,
                ParticleType::Lava => 30000,
                ParticleType::Ice => ICE_STANDARD_TEMP,
            },
            state: match ptype {
                ParticleType::Sand => SAND_STATE,
                ParticleType::MoltenGlass => MOLTENGLASS_STATE,
                ParticleType::Glass => GLASS_STATE,
                ParticleType::Water => WATER_STATE,
                ParticleType::Ice => ICE_STATE,
                ParticleType::Lava => ParticleState::Liquid,
            },
            density: match ptype {
                ParticleType::Sand => SAND_DENSITY,
                ParticleType::MoltenGlass => MOLTENGLASS_DENSITY,
                ParticleType::Glass => GLASS_DENSITY,
                ParticleType::Water => WATER_DENSITY,
                ParticleType::Ice => ICE_DENSITY,
                ParticleType::Lava => 300,
            },
            conductivity: match ptype {
                ParticleType::Sand => 20,
                ParticleType::MoltenGlass => 30,
                ParticleType::Glass => 20,
                ParticleType::Water => 70,
                ParticleType::Ice => 60,
                ParticleType::Lava => 30,
            },
            xd1: match ptype {
                ParticleType::Sand => SAND_STANDARD_XD1,
                ParticleType::MoltenGlass => MOLTENGLASS_STANDARD_XD1,
                ParticleType::Glass => GLASS_STANDARD_XD1,
                ParticleType::Water => WATER_STANDARD_XD1,
                ParticleType::Ice => ICE_STANDARD_XD1,
                ParticleType::Lava => 0,
            },
            xd2: match ptype {
                ParticleType::Sand => SAND_STANDARD_XD2,
                ParticleType::MoltenGlass => MOLTENGLASS_STANDARD_XD2,
                ParticleType::Glass => GLASS_STANDARD_XD2,
                ParticleType::Water => WATER_STANDARD_XD2,
                ParticleType::Ice => ICE_STANDARD_XD2,
                ParticleType::Lava => 0,
            },
            color_offset: (0, 0, 0), //TODO
            updateable: true,
        }
    }
}