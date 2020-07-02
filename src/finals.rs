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

pub const ICE_STANDARD_TEMP: u64 = 120;
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

pub const VAPORIZEDSILICON_STANDARD_TEMP: u64 = 2828;
pub const VAPORIZEDSILICON_STATE: ParticleState = ParticleState::Gas;
pub const VAPORIZEDSILICON_DENSITY: u16 = 0;
pub const VAPORIZEDSILICON_STANDARD_XD1: i8 = 0;
pub const VAPORIZEDSILICON_STANDARD_XD2: i8 = 0;
pub const VAPORIZEDSILICON_COLOR: (u8, u8, u8) = (255, 180, 220);

pub const OSMIUM_STANDARD_TEMP: u64 = 300;
pub const OSMIUM_STATE: ParticleState = ParticleState::Solid;
pub const OSMIUM_DENSITY: u16 = 22500;
pub const OSMIUM_STANDARD_XD1: i8 = 0;
pub const OSMIUM_STANDARD_XD2: i8 = 0;
pub const OSMIUM_COLOR: (u8, u8, u8) = (203, 205, 205);

pub const MOLTENOSMIUM_STANDARD_TEMP: u64 = 4700;
pub const MOLTENOSMIUM_STATE: ParticleState = ParticleState::Liquid;
pub const MOLTENOSMIUM_DENSITY: u16 = 20000;
pub const MOLTENOSMIUM_STANDARD_XD1: i8 = 0;
pub const MOLTENOSMIUM_STANDARD_XD2: i8 = 0;
pub const MOLTENOSMIUM_COLOR: (u8, u8, u8) = (240, 205, 205);

pub const VAPORISEDOSMIUM_STANDARD_TEMP: u64 = 5400;
pub const VAPORISEDOSMIUM_STATE: ParticleState = ParticleState::Gas;
pub const VAPORISEDOSMIUM_DENSITY: u16 = 0;
pub const VAPORISEDOSMIUM_STANDARD_XD1: i8 = 0;
pub const VAPORISEDOSMIUM_STANDARD_XD2: i8 = 0;
pub const VAPORISEDOSMIUM_COLOR: (u8, u8, u8) = (250, 200, 200);

pub const XTHERMIC_STANDARD_TEMP: u64 = 0;
pub const XTHERMIC_STATE: ParticleState = ParticleState::Powder;
pub const XTHERMIC_DENSITY: u16 = 1200;
pub const XTHERMIC_STANDARD_XD1: i8 = 0;
pub const XTHERMIC_STANDARD_XD2: i8 = 0;
pub const XTHERMIC_COLOR: (u8, u8, u8) = (100, 120, 255);

pub const MOLTENXTHERMIC_STANDARD_TEMP: u64 = 100;
pub const MOLTENXTHERMIC_STATE: ParticleState = ParticleState::Liquid;
pub const MOLTENXTHERMIC_DENSITY: u16 = 1500;
pub const MOLTENXTHERMIC_STANDARD_XD1: i8 = 0;
pub const MOLTENXTHERMIC_STANDARD_XD2: i8 = 0;
pub const MOLTENXTHERMIC_COLOR: (u8, u8, u8) = (100, 120, 225);

pub const VAPORISEDXTHERMIC_STANDARD_TEMP: u64 = 16500;
pub const VAPORISEDXTHERMIC_STATE: ParticleState = ParticleState::Gas;
pub const VAPORISEDXTHERMIC_DENSITY: u16 = 0;
pub const VAPORISEDXTHERMIC_STANDARD_XD1: i8 = 0;
pub const VAPORISEDXTHERMIC_STANDARD_XD2: i8 = 0;
pub const VAPORISEDXTHERMIC_COLOR: (u8, u8, u8) = (150, 120, 255);

pub const SUPERSOLIDWOLFRAM_STANDARD_TEMP: u64 = 300;
pub const SUPERSOLIDWOLFRAM_STATE: ParticleState = ParticleState::SuperSolid;
pub const SUPERSOLIDWOLFRAM_DENSITY: u16 = 65000;
pub const SUPERSOLIDWOLFRAM_STANDARD_XD1: i8 = 0;
pub const SUPERSOLIDWOLFRAM_STANDARD_XD2: i8 = 0;
pub const SUPERSOLIDWOLFRAM_COLOR: (u8, u8, u8) = (100, 100, 120);