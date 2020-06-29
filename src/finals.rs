use crate::ParticleState;

pub const SAND_STANDARD_TEMP: u64 = 300;
pub const SAND_STATE: ParticleState = ParticleState::Powder;
pub const SAND_DENSITY: u16 = 1400;
pub const SAND_STANDARD_XD1: i8 = 0;
pub const SAND_STANDARD_XD2: i8 = 0;
pub const SAND_COLOR: (u8, u8, u8) = (240,230,140);

pub const MOLTENGLASS_STANDARD_TEMP: u64 = 2200;
pub const MOLTENGLASS_STATE: ParticleState = ParticleState::Liquid;
pub const MOLTENGLASS_DENSITY: u16 = 2000;
pub const MOLTENGLASS_STANDARD_XD1: i8 = 0;
pub const MOLTENGLASS_STANDARD_XD2: i8 = 0;
pub const MOLTENGLASS_COLOR: (u8, u8, u8) = (255, 160, 160);

pub const GLASS_STANDARD_TEMP: u64 = 300;
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

pub const STEAM_STANDARD_TEMP: u64 = 500;
pub const STEAM_STATE: ParticleState = ParticleState::Gas;
pub const STEAM_DENSITY: u16 = 0;
pub const STEAM_STANDARD_XD1: i8 = 0;
pub const STEAM_STANDARD_XD2: i8 = 0;
pub const STEAM_COLOR: (u8, u8, u8) = (199, 213, 224);

pub const STONE_STANDARD_TEMP: u64 = 300;
pub const STONE_STATE: ParticleState = ParticleState::Solid;
pub const STONE_DENSITY: u16 = 1600;
pub const STONE_STANDARD_XD1: i8 = 0;
pub const STONE_STANDARD_XD2: i8 = 0;
pub const STONE_COLOR: (u8, u8, u8) = (149, 148, 139);

pub const LAVA_STANDARD_TEMP: u64 = 1700;
pub const LAVA_STATE: ParticleState = ParticleState::Liquid;
pub const LAVA_DENSITY: u16 = 3100;
pub const LAVA_STANDARD_XD1: i8 = 0;
pub const LAVA_STANDARD_XD2: i8 = 0;
pub const LAVA_COLOR: (u8, u8, u8) = (255, 30, 30);

pub const VAPORIZEDSILICON_STANDARD_TEMP: u64 = 2628;
pub const VAPORIZEDSILICON_STATE: ParticleState = ParticleState::Gas;
pub const VAPORIZEDSILICON_DENSITY: u16 = 0;
pub const VAPORIZEDSILICON_STANDARD_XD1: i8 = 0;
pub const VAPORIZEDSILICON_STANDARD_XD2: i8 = 0;
pub const VAPORIZEDSILICON_COLOR: (u8, u8, u8) = (255, 180, 220);