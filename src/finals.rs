use crate::ParticleState;

pub const SAND_STANDARD_TEMP: u64 = 0;
pub const SAND_STATE: ParticleState = ParticleState::Powder;
pub const SAND_DENSITY: u16 = 1400;
pub const SAND_STANDARD_XD1: i8 = 0;
pub const SAND_STANDARD_XD2: i8 = 0;
pub const SAND_COLOR: (u8, u8, u8) = (150, 130, 120);

pub const MOLTENGLASS_STANDARD_TEMP: u64 = 2200;
pub const MOLTENGLASS_STATE: ParticleState = ParticleState::Liquid;
pub const MOLTENGLASS_DENSITY: u16 = 2000;
pub const MOLTENGLASS_STANDARD_XD1: i8 = 0;
pub const MOLTENGLASS_STANDARD_XD2: i8 = 0;
pub const MOLTENGLASS_COLOR: (u8, u8, u8) = (255, 160, 160);

pub const GLASS_STANDARD_TEMP: u64 = 0;
pub const GLASS_STATE: ParticleState = ParticleState::Solid;
pub const GLASS_DENSITY: u16 = 2500;
pub const GLASS_STANDARD_XD1: i8 = 0;
pub const GLASS_STANDARD_XD2: i8 = 0;
pub const GLASS_COLOR: (u8, u8, u8) = (255, 255, 255);

pub const WATER_STANDARD_TEMP: u64 = 330;
pub const WATER_STATE: ParticleState = ParticleState::Liquid;
pub const WATER_DENSITY: u16 = 997;
pub const WATER_STANDARD_XD1: i8 = 0;
pub const WATER_STANDARD_XD2: i8 = 0;
pub const WATER_COLOR: (u8, u8, u8) = (0, 0, 255);

pub const ICE_STANDARD_TEMP: u64 = 0;
pub const ICE_STATE: ParticleState = ParticleState::Solid;
pub const ICE_DENSITY: u16 = 913;
pub const ICE_STANDARD_XD1: i8 = 0;
pub const ICE_STANDARD_XD2: i8 = 0;
pub const ICE_COLOR: (u8, u8, u8) = (195, 203, 217);