use crate::ParticleState;

pub const SAND_STANDARD_TEMP: u64 = 0;
pub const SAND_STATE: ParticleState = ParticleState::Powder;
pub const SAND_DENSITY: u16 = 200;
pub const SAND_STANDARD_XD1: i8 = 0;
pub const SAND_STANDARD_XD2: i8 = 0;
pub const SAND_COLOR: (u8, u8, u8) = (150, 130, 120);

pub const MOLTENGLASS_STANDARD_TEMP: u64 = 10000;
pub const MOLTENGLASS_STATE: ParticleState = ParticleState::Liquid;
pub const MOLTENGLASS_DENSITY: u16 = 150;
pub const MOLTENGLASS_STANDARD_XD1: i8 = 0;
pub const MOLTENGLASS_STANDARD_XD2: i8 = 0;
pub const MOLTENGLASS_COLOR: (u8, u8, u8) = (255, 160, 160);

pub const GLASS_STANDARD_TEMP: u64 = 0;
pub const GLASS_STATE: ParticleState = ParticleState::Solid;
pub const GLASS_DENSITY: u16 = 200;
pub const GLASS_STANDARD_XD1: i8 = 0;
pub const GLASS_STANDARD_XD2: i8 = 0;
pub const GLASS_COLOR: (u8, u8, u8) = (255, 255, 255);