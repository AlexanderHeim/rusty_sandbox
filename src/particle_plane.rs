use crate::{GRID_SLOT_AMOUNT, GRID_SLOT_SIZE};
use crate::particle_behaviour::*;
use crate::finals::*;

use sdl2::pixels::Color;
use rand::prelude::*;

pub struct ParticlePlane {
    pub grid: Box<[[Option<Particle>; GRID_SLOT_AMOUNT.1 as usize]; GRID_SLOT_AMOUNT.0 as usize]>,
    pub rng: ThreadRng,
}

impl ParticlePlane {
    pub fn new() -> Self {
        ParticlePlane { grid: Box::new([[None; GRID_SLOT_AMOUNT.1 as usize]; GRID_SLOT_AMOUNT.0 as usize]), rng: rand::thread_rng() }
    }

    pub fn update(&mut self) {
        for x in 0..GRID_SLOT_AMOUNT.0 as usize {
            for y in 0..GRID_SLOT_AMOUNT.1 as usize {
                if self.grid[x][y].is_some() {
                    transfer_heat(self, x, y);
                    update_specific_particle(self, x, y);
                }
                
            }
        }
        for x in 0..GRID_SLOT_AMOUNT.0 as usize {
            for y in 0..GRID_SLOT_AMOUNT.1 as usize {
                if self.grid[x][y].is_some() {
                    if self.grid[x][y].unwrap().updateable {
                        match self.grid[x][y].unwrap().ptype {
                            ParticleType::Sand => move_powder(self, x, y),
                            ParticleType::MoltenGlass => move_liquid(self, x, y),
                            ParticleType::Glass => move_solid(self, x, y),
                            ParticleType::Water => move_liquid(self, x, y),
                            ParticleType::Ice => move_solid(self, x, y),
                            ParticleType::Steam => move_gas(self, x, y),
                            ParticleType::Stone => move_solid(self, x, y),
                            ParticleType::Lava => move_liquid(self, x, y),
                            ParticleType::VaporisedSilicon => move_gas(self, x, y),
                            ParticleType::Osmium => move_solid(self, x, y),
                            ParticleType::MoltenOsmium => move_liquid(self, x, y),
                            ParticleType::VaporisedOsmium => move_gas(self, x, y),
                            ParticleType::XThermic => move_powder(self, x, y),
                            ParticleType::MoltenXThermic => move_liquid(self, x, y),
                            ParticleType::VaporisedXThermic => move_gas(self, x, y),
                            ParticleType::SuperSolidWolfram => (),
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
                        ParticleType::Steam => STEAM_COLOR,
                        ParticleType::Stone => STONE_COLOR,
                        ParticleType::Lava => LAVA_COLOR,
                        ParticleType::VaporisedSilicon => VAPORIZEDSILICON_COLOR,
                        ParticleType::Osmium => OSMIUM_COLOR,
                        ParticleType::MoltenOsmium => MOLTENOSMIUM_COLOR,
                        ParticleType::VaporisedOsmium => VAPORISEDOSMIUM_COLOR,
                        ParticleType::XThermic => XTHERMIC_COLOR,
                        ParticleType::MoltenXThermic => MOLTENXTHERMIC_COLOR,
                        ParticleType::VaporisedXThermic => VAPORISEDXTHERMIC_COLOR,
                        ParticleType::SuperSolidWolfram => SUPERSOLIDWOLFRAM_COLOR,
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
    Steam,
    Stone,
    Lava,
    VaporisedSilicon,
    Osmium,
    MoltenOsmium,
    VaporisedOsmium,
    XThermic,
    MoltenXThermic,
    VaporisedXThermic,
    SuperSolidWolfram,
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
                ParticleType::Ice => ICE_STANDARD_TEMP,
                ParticleType::Steam => STEAM_STANDARD_TEMP,
                ParticleType::Stone => STONE_STANDARD_TEMP,
                ParticleType::Lava => LAVA_STANDARD_TEMP,
                ParticleType::VaporisedSilicon => VAPORIZEDSILICON_STANDARD_TEMP,
                ParticleType::Osmium => OSMIUM_STANDARD_TEMP,
                ParticleType::MoltenOsmium => MOLTENOSMIUM_STANDARD_TEMP,
                ParticleType::VaporisedOsmium => VAPORISEDOSMIUM_STANDARD_TEMP,
                ParticleType::XThermic => XTHERMIC_STANDARD_TEMP,
                ParticleType::MoltenXThermic => MOLTENXTHERMIC_STANDARD_TEMP,
                ParticleType::VaporisedXThermic => VAPORISEDXTHERMIC_STANDARD_TEMP,
                ParticleType::SuperSolidWolfram => SUPERSOLIDWOLFRAM_STANDARD_TEMP,
                
            },
            state: match ptype {
                ParticleType::Sand => SAND_STATE,
                ParticleType::MoltenGlass => MOLTENGLASS_STATE,
                ParticleType::Glass => GLASS_STATE,
                ParticleType::Water => WATER_STATE,
                ParticleType::Ice => ICE_STATE,
                ParticleType::Steam => STEAM_STATE,
                ParticleType::Stone => STONE_STATE,
                ParticleType::Lava => LAVA_STATE,
                ParticleType::VaporisedSilicon => VAPORIZEDSILICON_STATE,
                ParticleType::Osmium => OSMIUM_STATE,
                ParticleType::MoltenOsmium => MOLTENOSMIUM_STATE,
                ParticleType::VaporisedOsmium => VAPORISEDOSMIUM_STATE,
                ParticleType::XThermic => XTHERMIC_STATE,
                ParticleType::MoltenXThermic => MOLTENXTHERMIC_STATE,
                ParticleType::VaporisedXThermic => VAPORISEDXTHERMIC_STATE,
                ParticleType::SuperSolidWolfram => SUPERSOLIDWOLFRAM_STATE,
            },
            density: match ptype {
                ParticleType::Sand => SAND_DENSITY,
                ParticleType::MoltenGlass => MOLTENGLASS_DENSITY,
                ParticleType::Glass => GLASS_DENSITY,
                ParticleType::Water => WATER_DENSITY,
                ParticleType::Ice => ICE_DENSITY,
                ParticleType::Steam => STEAM_DENSITY,
                ParticleType::Stone => STONE_DENSITY,
                ParticleType::Lava => LAVA_DENSITY,
                ParticleType::VaporisedSilicon => VAPORIZEDSILICON_DENSITY,
                ParticleType::Osmium => OSMIUM_DENSITY,
                ParticleType::MoltenOsmium => MOLTENOSMIUM_DENSITY,
                ParticleType::VaporisedOsmium => VAPORISEDOSMIUM_DENSITY,
                ParticleType::XThermic => XTHERMIC_DENSITY,
                ParticleType::MoltenXThermic => MOLTENXTHERMIC_DENSITY,
                ParticleType::VaporisedXThermic => VAPORISEDXTHERMIC_DENSITY,
                ParticleType::SuperSolidWolfram => SUPERSOLIDWOLFRAM_DENSITY,
            },
            conductivity: match ptype {
                ParticleType::Sand => 20,
                ParticleType::MoltenGlass => 30,
                ParticleType::Glass => 20,
                ParticleType::Water => 70,
                ParticleType::Ice => 60,
                ParticleType::Steam => 90,
                ParticleType::Stone => 45,
                ParticleType::Lava => 60,
                ParticleType::VaporisedSilicon => 90,
                ParticleType::Osmium => 80,
                ParticleType::MoltenOsmium => 85,
                ParticleType::VaporisedOsmium => 99,
                ParticleType::XThermic => 100,
                ParticleType::MoltenXThermic => 100,
                ParticleType::VaporisedXThermic => 100,
                ParticleType::SuperSolidWolfram => 70,
            },
            xd1: match ptype {
                ParticleType::Sand => SAND_STANDARD_XD1,
                ParticleType::MoltenGlass => MOLTENGLASS_STANDARD_XD1,
                ParticleType::Glass => GLASS_STANDARD_XD1,
                ParticleType::Water => WATER_STANDARD_XD1,
                ParticleType::Ice => ICE_STANDARD_XD1,
                ParticleType::Steam => STEAM_STANDARD_XD1,
                ParticleType::Stone => STONE_STANDARD_XD1,
                ParticleType::Lava => LAVA_STANDARD_XD1,
                ParticleType::VaporisedSilicon => VAPORIZEDSILICON_STANDARD_XD1,
                ParticleType::Osmium => OSMIUM_STANDARD_XD1,
                ParticleType::MoltenOsmium => MOLTENOSMIUM_STANDARD_XD1,
                ParticleType::VaporisedOsmium => VAPORISEDOSMIUM_STANDARD_XD1,
                ParticleType::XThermic => XTHERMIC_STANDARD_XD1,
                ParticleType::MoltenXThermic => MOLTENXTHERMIC_STANDARD_XD1,
                ParticleType::VaporisedXThermic => VAPORISEDXTHERMIC_STANDARD_XD1,
                ParticleType::SuperSolidWolfram => SUPERSOLIDWOLFRAM_STANDARD_XD1,
            },
            xd2: match ptype {
                ParticleType::Sand => SAND_STANDARD_XD2,
                ParticleType::MoltenGlass => MOLTENGLASS_STANDARD_XD2,
                ParticleType::Glass => GLASS_STANDARD_XD2,
                ParticleType::Water => WATER_STANDARD_XD2,
                ParticleType::Ice => ICE_STANDARD_XD2,
                ParticleType::Steam => STEAM_STANDARD_XD2,
                ParticleType::Stone => STONE_STANDARD_XD2,
                ParticleType::Lava => LAVA_STANDARD_XD2,
                ParticleType::VaporisedSilicon => VAPORIZEDSILICON_STANDARD_XD2,
                ParticleType::Osmium => OSMIUM_STANDARD_XD2,
                ParticleType::MoltenOsmium => MOLTENOSMIUM_STANDARD_XD2,
                ParticleType::VaporisedOsmium => VAPORISEDOSMIUM_STANDARD_XD2,
                ParticleType::XThermic => XTHERMIC_STANDARD_XD2,
                ParticleType::MoltenXThermic => MOLTENXTHERMIC_STANDARD_XD2,
                ParticleType::VaporisedXThermic => VAPORISEDXTHERMIC_STANDARD_XD2,
                ParticleType::SuperSolidWolfram => SUPERSOLIDWOLFRAM_STANDARD_XD2,
            },
            color_offset: (0, 0, 0), //TODO
            updateable: true,
        }
    }
}