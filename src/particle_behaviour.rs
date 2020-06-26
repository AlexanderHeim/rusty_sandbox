use crate::particle_plane::*;
use crate::GRID_SLOT_AMOUNT;
use crate::finals::*;

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

fn switch_particles(plane: &mut ParticlePlane, x: usize, y: usize, x_1: usize, y_1: usize) {
    let mut temp_particle = plane.grid[x][y].take();
    plane.grid[x][y] = plane.grid[x_1][y_1].take();
    plane.grid[x_1][y_1] = temp_particle.take();
    plane.grid[x_1][y_1].as_mut().unwrap().updateable = false;
    plane.grid[x][y].as_mut().unwrap().updateable = false;
}

pub fn transfer_heat(plane: &mut ParticlePlane, x: usize, y: usize){
    let temp_batch = plane.grid[x][y].unwrap().temp/8;
    //Transfer UP
    if y > 0 && plane.grid[x][y - 1].is_some() {
        plane.grid[x][y - 1].as_mut().unwrap().temp += temp_batch;
        plane.grid[x][y].as_mut().unwrap().temp -= temp_batch;
    }
    // Transfer DOWN
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 && plane.grid[x][y + 1].is_some() {
        plane.grid[x][y + 1].as_mut().unwrap().temp += temp_batch;
        plane.grid[x][y].as_mut().unwrap().temp -= temp_batch;
    }
    // Transfer LEFT
    if x > 0 && plane.grid[x - 1][y].is_some() {
        plane.grid[x - 1][y].as_mut().unwrap().temp += temp_batch;
        plane.grid[x][y].as_mut().unwrap().temp -= temp_batch;
    }
    // Transfer RIGHT
    if x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y].is_some() {
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

pub fn update_specific_particle(plane: &mut ParticlePlane, x: usize, y: usize) {
    match plane.grid[x][y].unwrap().ptype {
        ParticleType::Sand => {
            if plane.grid[x][y].unwrap().temp > 8000 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenGlass;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENGLASS_DENSITY;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENGLASS_STATE;
            }
        },
        ParticleType::MoltenGlass => {
            if plane.grid[x][y].unwrap().temp < 8000 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::Glass;
                plane.grid[x][y].as_mut().unwrap().state = GLASS_STATE;
                plane.grid[x][y].as_mut().unwrap().density = GLASS_DENSITY;
            }
        },
        ParticleType::Glass => {
            if plane.grid[x][y].unwrap().temp > 8000 {
                plane.grid[x][y].as_mut().unwrap().ptype = ParticleType::MoltenGlass;
                plane.grid[x][y].as_mut().unwrap().state = MOLTENGLASS_STATE;
                plane.grid[x][y].as_mut().unwrap().density = MOLTENGLASS_DENSITY;
            }
        }
        ParticleType::Water => {

        },
        ParticleType::Lava => {

        },
    }
}