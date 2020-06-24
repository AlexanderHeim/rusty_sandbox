use crate::particle_plane::*;
use crate::GRID_SLOT_AMOUNT;

pub fn move_powder(plane: &mut ParticlePlane, x: usize, y: usize) {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 {
        //FALL IF NONE BELOW
        if plane.grid[x][y + 1].is_none() {
            plane.grid[x][y + 1] = plane.grid[x][y].take();
            plane.grid[x][y + 1].as_mut().unwrap().updateable = false;
            return
        }

        //SWITCH WITH LIGHTER LIQUID/GAS
        if (plane.grid[x][y + 1].unwrap().state == ParticleState::Liquid || plane.grid[x][y + 1].unwrap().state == ParticleState::Gas) && plane.grid[x][y + 1].unwrap().density <= plane.grid[x][y].unwrap().density {
            let mut temp_particle = plane.grid[x][y].take();
            plane.grid[x][y] = plane.grid[x][y + 1].take();
            plane.grid[x][y + 1] = temp_particle.take();
            plane.grid[x][y + 1].as_mut().unwrap().updateable = false;
            return
        }


        if rand::random() {
            //FALL DOWN LEFT IF POSSIBLE
            if x > 0 && plane.grid[x - 1][y + 1].is_none() {
                plane.grid[x - 1][y + 1] = plane.grid[x][y].take();
                plane.grid[x - 1][y + 1].as_mut().unwrap().updateable = false;
                return
            }
        } else {
            //FALL DOWN RIGHT IF POSSIBLE
            if x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y + 1].is_none() {
                plane.grid[x + 1][y + 1] = plane.grid[x][y].take();
                plane.grid[x + 1][y + 1].as_mut().unwrap().updateable = false;
                return
            }
        }
    }
}

pub fn move_liquid(plane: &mut ParticlePlane, x: usize, y: usize) {
    if y < GRID_SLOT_AMOUNT.1 as usize - 1 {
        //FALL IF NONE BELOW
        if plane.grid[x][y + 1].is_none() {
            plane.grid[x][y + 1] = plane.grid[x][y].take();
            plane.grid[x][y + 1].as_mut().unwrap().updateable = false;
            return
        }

        //SWITCH WITH LIGHTER LIQUID/GAS
        if plane.grid[x][y + 1].unwrap().ptype != ParticleType::Water && (plane.grid[x][y + 1].unwrap().state == ParticleState::Liquid || plane.grid[x][y + 1].unwrap().state == ParticleState::Gas) && plane.grid[x][y + 1].unwrap().density < plane.grid[x][y].unwrap().density {
            let mut temp_particle = plane.grid[x][y].take();
            plane.grid[x][y] = plane.grid[x][y + 1].take();
            plane.grid[x][y + 1] = temp_particle.take();
            plane.grid[x][y + 1].as_mut().unwrap().updateable = false;
            return
        }

        
        if rand::random() {
            //FALL DOWN LEFT IF POSSIBLE
            if x > 0 && plane.grid[x - 1][y + 1].is_none() {
                plane.grid[x - 1][y + 1] = plane.grid[x][y].take();
                plane.grid[x - 1][y + 1].as_mut().unwrap().updateable = false;
                return
            }
        } else {
            //FALL DOWN RIGHT IF POSSIBLE
            if x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y + 1].is_none() {
                plane.grid[x + 1][y + 1] = plane.grid[x][y].take();
                plane.grid[x + 1][y + 1].as_mut().unwrap().updateable = false;
                return
            }
        }

        if rand::random(){
            //MOVE LEFT IF POSSIBLE
            if x > 0 && plane.grid[x - 1][y].is_none() {
                plane.grid[x - 1][y] = plane.grid[x][y].take();
                plane.grid[x - 1][y].as_mut().unwrap().updateable = false;
                return
            }
            //MOVE RIGHT IF POSSIBLE
            if x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y].is_none() {
                plane.grid[x + 1][y] = plane.grid[x][y].take();
                plane.grid[x + 1][y].as_mut().unwrap().updateable = false;
                return
            }
        } else {
            //MOVE RIGHT IF POSSIBLE
            if x < GRID_SLOT_AMOUNT.0 as usize - 1 && plane.grid[x + 1][y].is_none() {
                plane.grid[x + 1][y] = plane.grid[x][y].take();
                plane.grid[x + 1][y].as_mut().unwrap().updateable = false;
                return
            }
            //MOVE LEFT IF POSSIBLE
            if x > 0 && plane.grid[x - 1][y].is_none() {
                plane.grid[x - 1][y] = plane.grid[x][y].take();
                plane.grid[x - 1][y].as_mut().unwrap().updateable = false;
                return
            }
        }
        
    }
}