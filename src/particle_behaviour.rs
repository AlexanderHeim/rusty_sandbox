use crate::particle_plane::*;
use crate::GRID_SLOT_AMOUNT;
use crate::finals::*;
use rand::*;

pub fn move_powder(plane: &mut ParticlePlane, x: usize, y: usize) {
    if rule_1(plane, x, y) { return }
    if rule_6(plane, x, y) { return }
    if rule_2(plane, x, y) { return }
    if rule_9(plane, x, y) { return }
}

pub fn move_liquid(plane: &mut ParticlePlane, x: usize, y: usize) {
    if rule_1(plane, x, y) { return }
    if rule_6(plane, x, y) { return }
    if rule_2(plane, x, y) { return }
    if rule_9(plane, x, y) { return }
    if rule_5(plane, x, y) { return }
    if rule_12(plane, x, y) { return }
}

pub fn move_solid(plane: &mut ParticlePlane, x: usize, y: usize) {
    if rule_1(plane, x, y) { return }
    if rule_6(plane, x, y) { return }
}

pub fn move_gas(plane: &mut ParticlePlane, x: usize, y: usize) {
    let factor = plane.rng.gen_range(9994, 10000) as f64 / 10000 as f64;
    if rand::random() { plane.grid[x][y].as_mut().unwrap().temp = (plane.grid[x][y].unwrap().temp as f64 * factor) as u64; }
    if rule_13(plane, x, y) { return }
}

fn switch_particles(plane: &mut ParticlePlane, x: usize, y: usize, x_1: usize, y_1: usize) {
    let mut temp_particle = plane.grid[x][y].take();
    plane.grid[x][y] = plane.grid[x_1][y_1].take();
    plane.grid[x_1][y_1] = temp_particle.take();
    plane.grid[x_1][y_1].as_mut().unwrap().updateable = false;
    plane.grid[x][y].as_mut().unwrap().updateable = false;
}

pub fn transfer_heat(plane: &mut ParticlePlane, x: usize, y: usize){
    //Transfer UP
    if y > 0 && plane.grid[x][y - 1].is_some() && plane.grid[x][y - 1].unwrap().temp < plane.grid[x][y].unwrap().temp {
        let temp_batch = ((plane.grid[x][y].unwrap().temp - plane.grid[x][y - 1].unwrap().temp)*(plane.grid[x][y].unwrap().conductivity + plane.grid[x][y - 1].unwrap().conductivity) as u64)/800;
        plane.grid[x][y - 1].as_mut().unwrap().temp += temp_batch;
        plane.grid[x][y].as_mut().unwrap().temp -= temp_batch;
    }
    // Transfer DOWN
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && plane.grid[x][y + 1].is_some()  && plane.grid[x][y + 1].unwrap().temp < plane.grid[x][y].unwrap().temp{
        let temp_batch = ((plane.grid[x][y].unwrap().temp - plane.grid[x][y + 1].unwrap().temp)*(plane.grid[x][y].unwrap().conductivity + plane.grid[x][y + 1].unwrap().conductivity) as u64)/800;
        plane.grid[x][y + 1].as_mut().unwrap().temp += temp_batch;
        plane.grid[x][y].as_mut().unwrap().temp -= temp_batch;
    }
    // Transfer LEFT
    if x > 0 && plane.grid[x - 1][y].is_some()  && plane.grid[x - 1][y].unwrap().temp < plane.grid[x][y].unwrap().temp{
        let temp_batch = ((plane.grid[x][y].unwrap().temp - plane.grid[x - 1][y].unwrap().temp)*(plane.grid[x][y].unwrap().conductivity + plane.grid[x - 1][y].unwrap().conductivity) as u64)/800;
        plane.grid[x - 1][y].as_mut().unwrap().temp += temp_batch;
        plane.grid[x][y].as_mut().unwrap().temp -= temp_batch;
    }
    // Transfer RIGHT
    if x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y].is_some()  && plane.grid[x + 1][y].unwrap().temp < plane.grid[x][y].unwrap().temp{
        let temp_batch = ((plane.grid[x][y].unwrap().temp - plane.grid[x + 1][y].unwrap().temp)*(plane.grid[x][y].unwrap().conductivity + plane.grid[x + 1][y].unwrap().conductivity) as u64)/800;
        plane.grid[x + 1][y].as_mut().unwrap().temp += temp_batch;
        plane.grid[x][y].as_mut().unwrap().temp -= temp_batch;
    }
    
}
//RULE 1
// FALL ONE IF NONE BELOW
fn rule_1(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && plane.grid[x][y + 1].is_none() {
        plane.grid[x][y + 1] = plane.grid[x][y].take();
        plane.grid[x][y + 1].as_mut().unwrap().updateable = false;
        return true
    }
    return false
}

//RULE 2
// FALL ONE DOWN AND ONE LEFT/RIGHT IF ONE BELOW AND DOWN LEFT/RIGHT IS NONE
// SLOW VERSION
fn rule_2(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y >= GRID_SLOT_AMOUNT.1 as usize - 1 { return false}
    if rand::random() {
        if x > 0 && plane.grid[x - 1][y + 1].is_none() {
            plane.grid[x - 1][y + 1] = plane.grid[x][y].take();
            plane.grid[x - 1][y + 1].as_mut().unwrap().updateable = false;
            return true
        }
    } else {
        if x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y + 1].is_none() {
            plane.grid[x + 1][y + 1] = plane.grid[x][y].take();
            plane.grid[x + 1][y + 1].as_mut().unwrap().updateable = false;
            return true
        }
    }
    return false
}

//RULE 3
// IF SOME BELOW MOVE LEFT IF LEFT IS NONE
fn rule_3(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && x > 0 && plane.grid[x - 1][y].is_none() && plane.grid[x][y + 1].is_some() {
        plane.grid[x - 1][y] = plane.grid[x][y].take();
        plane.grid[x - 1][y].as_mut().unwrap().updateable = false;
        return true
    }
    if y == GRID_SLOT_AMOUNT.1 as usize - 1 && x > 0 && plane.grid[x - 1][y].is_none() {
        plane.grid[x - 1][y] = plane.grid[x][y].take();
        plane.grid[x - 1][y].as_mut().unwrap().updateable = false;
        return true
    }
    return false
}

//RULE 4
// IF SOME BELOW MOVE RIGHT IF RIGHT IS NONE
fn rule_4(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y].is_none() && plane.grid[x][y + 1].is_some() {
        plane.grid[x + 1][y] = plane.grid[x][y].take();
        plane.grid[x + 1][y].as_mut().unwrap().updateable = false;
        return true
    }
    if y == GRID_SLOT_AMOUNT.1 as usize - 1 && x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y].is_none() {
        plane.grid[x + 1][y] = plane.grid[x][y].take();
        plane.grid[x + 1][y].as_mut().unwrap().updateable = false;
        return true
    }
    return false
}

//RULE 5
// MOVE LEFT/RIGHT IF SOME BELOW
// FAST VERSION
fn rule_5(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 {
        if rand::random() {
            if rule_3(plane, x, y) { return true }
            if rule_4(plane, x, y) { return true }
        } else {
            if rule_4(plane, x, y) { return true }
            if rule_3(plane, x, y) { return true }
        }
    }
    return false
}
//RULE 6
// SWITCH WITH LIGTHER LIQUID/GAS BELOW
fn rule_6(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && plane.grid[x][y + 1].is_some() && ( plane.grid[x][y + 1].unwrap().state == ParticleState::Liquid ||  plane.grid[x][y + 1].unwrap().state == ParticleState::Gas) && plane.grid[x][y + 1].unwrap().density < plane.grid[x][y].unwrap().density {
        switch_particles(plane, x, y, x, y + 1);
        return true
    }
    return false
}

//RULE 7
// SWITCH WITH LIGTHER LIQUID/GAS BELOW LEFT
fn rule_7(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && x > 0 && plane.grid[x - 1][y + 1].is_some() && ( plane.grid[x - 1][y + 1].unwrap().state == ParticleState::Liquid ||  plane.grid[x - 1][y + 1].unwrap().state == ParticleState::Gas) && plane.grid[x - 1][y + 1].unwrap().density < plane.grid[x][y].unwrap().density {
        switch_particles(plane, x, y, x - 1, y + 1);
        return true
    }
    return false
}

//RULE 8
// SWITCH WITH LIGTHER LIQUID/GAS BELOW RIGHT
fn rule_8(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y + 1].is_some() && ( plane.grid[x + 1][y + 1].unwrap().state == ParticleState::Liquid ||  plane.grid[x + 1][y + 1].unwrap().state == ParticleState::Gas) && plane.grid[x + 1][y + 1].unwrap().density < plane.grid[x][y].unwrap().density {
        switch_particles(plane, x, y, x + 1, y + 1);
        return true
    }
    return false
}

//RULE 9
// SWITCH WITH LIGTHER LIQUID/GAS BELOW LEFT/RIGHT
// SLOW VERSION
fn rule_9(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if y >= GRID_SLOT_AMOUNT.1 as usize - 1 { return false}
    if rand::random() {
        return rule_7(plane, x, y);
    } else {
        return rule_8(plane, x, y);
    }
}

//RULE 10
// SWITCH WITH LIGTHER LIQUID/GAS LEFT
fn rule_10(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if x == 0 { return false}
    if plane.grid[x - 1][y].is_some() && (plane.grid[x - 1][y].unwrap().state == ParticleState::Liquid || plane.grid[x - 1][y].unwrap().state == ParticleState::Gas) && plane.grid[x - 1][y].unwrap().density < plane.grid[x][y].unwrap().density {
        switch_particles(plane, x, y, x - 1, y);
        return true;
    }
    return false;
}

//RULE 11
// SWITCH WITH LIGTHER LIQUID/GAS RIGHT
fn rule_11(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if x >= GRID_SLOT_AMOUNT.0 as usize - 1 { return false}
    if plane.grid[x + 1][y].is_some() && (plane.grid[x + 1][y].unwrap().state == ParticleState::Liquid || plane.grid[x + 1][y].unwrap().state == ParticleState::Gas) && plane.grid[x + 1][y].unwrap().density < plane.grid[x][y].unwrap().density {
        switch_particles(plane, x, y, x + 1, y);
        return true;
    }
    return false;
}

//RULE 12
// SWITCH WITH LIGTHER LIQUID/GAS LEFT/RIGHT
// SLOW VERSION
fn rule_12(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if rand::random() {
        return rule_10(plane, x, y);
    } else {
        return rule_11(plane, x, y);
    }
}

//RULE 13
// MOVE RANDOMLY AND SWITCH WITH GASES RANDOMLY
// SLOW VERSION
fn rule_13(plane: &mut ParticlePlane, x: usize, y: usize) -> bool {
    if rand::random() {
        let r1 = plane.rng.gen_range(0, 3);
        let r2 = plane.rng.gen_range(0, 3);
        if x + r1 -1 > 0 && x + r1 -1 < GRID_SLOT_AMOUNT.0 as usize - 1 && y + r2 -1 > 0 && y + r2 -1 < GRID_SLOT_AMOUNT.1 as usize - 1 {
            if plane.grid[x + r1 -1][y + r2 - 1].is_none() {
                plane.grid[x][y].as_mut().unwrap().updateable = false;
                plane.grid[x + r1 - 1][y + r2 -1] = plane.grid[x][y].take();
                return true;
            } else if plane.grid[x + r1 -1][y + r2 -1].unwrap().state == ParticleState::Gas {
                switch_particles(plane, x, y, x + r1 -1, y + r2 -1);
                return true;
            }
        }
        return false;
    }
    return false;
}

pub fn update_specific_particle(plane: &mut ParticlePlane, x: usize, y: usize) {
    match plane.grid[x][y].unwrap().ptype {
        ParticleType::Sand => {
            if plane.grid[x][y].unwrap().temp > 1900 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenGlass;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENGLASS_DENSITY;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENGLASS_STATE;
            }
        },
        ParticleType::MoltenGlass => {
            if plane.grid[x][y].unwrap().temp < 1900 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Glass;
                plane.grid[x][y].as_mut().unwrap().state = GLASS_STATE;
                plane.grid[x][y].as_mut().unwrap().density = GLASS_DENSITY;
            }
            if plane.grid[x][y].unwrap().temp > 2628 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::VaporisedSilicon;
                plane.grid[x][y].as_mut().unwrap().state = VAPORIZEDSILICON_STATE;
                plane.grid[x][y].as_mut().unwrap().density = VAPORIZEDSILICON_DENSITY;
            }
        },
        ParticleType::Glass => {
            if plane.grid[x][y].unwrap().temp > 1900 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenGlass;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENGLASS_STATE;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENGLASS_DENSITY;
            }
        },
        ParticleType::Water => {
            if plane.grid[x][y].unwrap().temp < 273 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Ice;
                plane.grid[x][y].as_mut().unwrap().state = ICE_STATE;
                plane.grid[x][y].as_mut().unwrap().density = ICE_DENSITY;
                return;
            }
            if plane.grid[x][y].unwrap().temp > 373 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Steam;
                plane.grid[x][y].as_mut().unwrap().state = STEAM_STATE;
                plane.grid[x][y].as_mut().unwrap().density = STEAM_DENSITY;
                return;
            }
        },
        ParticleType::Ice => {
            if plane.grid[x][y].unwrap().temp > 273 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Water;
                plane.grid[x][y].as_mut().unwrap().state = WATER_STATE;
                plane.grid[x][y].as_mut().unwrap().density = WATER_DENSITY;
            }
        },
        ParticleType::Steam => {
            if plane.grid[x][y].unwrap().temp < 373 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Water;
                plane.grid[x][y].as_mut().unwrap().state = WATER_STATE;
                plane.grid[x][y].as_mut().unwrap().density = WATER_DENSITY;
            }
        },
        ParticleType::Lava => {
            if plane.grid[x][y].unwrap().temp < 800 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Stone;
                plane.grid[x][y].as_mut().unwrap().state = STONE_STATE;
                plane.grid[x][y].as_mut().unwrap().density = STONE_DENSITY;
                return;
            }
            if plane.grid[x][y].unwrap().temp > 2628 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::VaporisedSilicon;
                plane.grid[x][y].as_mut().unwrap().state = VAPORIZEDSILICON_STATE;
                plane.grid[x][y].as_mut().unwrap().density = VAPORIZEDSILICON_DENSITY;
                return;
            }
        },
        ParticleType::Stone => {
            if plane.grid[x][y].unwrap().temp > 800 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Lava;
                plane.grid[x][y].as_mut().unwrap().state = LAVA_STATE;
                plane.grid[x][y].as_mut().unwrap().density = LAVA_DENSITY;
            }
        },
        ParticleType::VaporisedSilicon => {
            if plane.grid[x][y].unwrap().temp < 2628 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Lava;
                plane.grid[x][y].as_mut().unwrap().state = LAVA_STATE;
                plane.grid[x][y].as_mut().unwrap().density = LAVA_DENSITY;
            }
        },
        ParticleType::Osmium => {
            if plane.grid[x][y].unwrap().temp > 3306 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenOsmium;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENOSMIUM_STATE;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENOSMIUM_DENSITY;
            }
        },
        ParticleType::MoltenOsmium => {
            if plane.grid[x][y].unwrap().temp < 3306 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Osmium;
                plane.grid[x][y].as_mut().unwrap().state = OSMIUM_STATE;
                plane.grid[x][y].as_mut().unwrap().density = OSMIUM_DENSITY;
                return;
            }
            if plane.grid[x][y].unwrap().temp > 5285 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::VaporisedOsmium;
                plane.grid[x][y].as_mut().unwrap().state = VAPORISEDOSMIUM_STATE;
                plane.grid[x][y].as_mut().unwrap().density = VAPORISEDOSMIUM_DENSITY;
                return;
            }
        },
        ParticleType::VaporisedOsmium => {
            if plane.grid[x][y].unwrap().temp < 5285 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenOsmium;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENOSMIUM_STATE;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENOSMIUM_DENSITY;
            }
        },
        ParticleType::XThermic => {
            if plane.grid[x][y].unwrap().temp > 10 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenXThermic;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENXTHERMIC_STATE;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENXTHERMIC_DENSITY;
            }
        },
        ParticleType::MoltenXThermic => {
            if plane.grid[x][y].unwrap().temp < 10 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::XThermic;
                plane.grid[x][y].as_mut().unwrap().state = XTHERMIC_STATE;
                plane.grid[x][y].as_mut().unwrap().density = XTHERMIC_DENSITY;
                return;
            }
            if plane.grid[x][y].unwrap().temp > 14000 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::VaporisedXThermic;
                plane.grid[x][y].as_mut().unwrap().state = VAPORISEDXTHERMIC_STATE;
                plane.grid[x][y].as_mut().unwrap().density = VAPORISEDXTHERMIC_DENSITY;
                return;
            }
        },
        ParticleType::VaporisedXThermic => {
            if plane.grid[x][y].unwrap().temp < 14000 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenXThermic;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENXTHERMIC_STATE;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENXTHERMIC_DENSITY;
            }
        },
        ParticleType::SuperSolidWolfram => {
            //TODO: ADD MOLTEN WOLFRAM
            if plane.grid[x][y].unwrap().temp > 3695 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenOsmium;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENOSMIUM_STATE;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENOSMIUM_DENSITY;
            }
        },
    }
}