extern crate sdl2; 

mod particle_plane;
mod particle_behaviour;

use particle_plane::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

const GRID_SLOT_SIZE: (i32, i32) = (3, 3);
pub const GRID_SLOT_AMOUNT: (i32, i32) = (300, 300);
const WINDOW_SIZE: (f32, f32) = (GRID_SLOT_SIZE.0 as f32 * GRID_SLOT_AMOUNT.0 as f32, GRID_SLOT_SIZE.1 as f32 * GRID_SLOT_AMOUNT.1 as f32);

pub fn main() -> Result<(), String> {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_SIZE.0 as u32, WINDOW_SIZE.1 as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut start = Instant::now();

    let mut pplane = ParticlePlane::new();

    'running: loop {
        if event_pump.mouse_state().left() {
            let x = get_cursor_pos(&event_pump).0;
            let y = get_cursor_pos(&event_pump).1;
            if pplane.grid[(x/GRID_SLOT_SIZE.0) as usize][(y/GRID_SLOT_SIZE.1) as usize].is_none() {
                pplane.grid[(x/GRID_SLOT_SIZE.0) as usize][(y/GRID_SLOT_SIZE.1) as usize] = Some(Particle::new(ParticleType::Sand));
            }
        }

        if event_pump.mouse_state().right() {
            let x = get_cursor_pos(&event_pump).0;
            let y = get_cursor_pos(&event_pump).1;
            if pplane.grid[(x/GRID_SLOT_SIZE.0) as usize][(y/GRID_SLOT_SIZE.1) as usize].is_none() {
                pplane.grid[(x/GRID_SLOT_SIZE.0) as usize][(y/GRID_SLOT_SIZE.1) as usize] = Some(Particle::new(ParticleType::Water));
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        if start.elapsed().as_millis() > 30 {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // CODE
            pplane.reset_updateable();
            pplane.update();
            pplane.draw(&mut canvas);

            canvas.present();
            start = Instant::now();
        }
        
        
    }
    Ok(())
}

fn get_cursor_pos(e: &sdl2::EventPump) -> (i32, i32){
    (e.mouse_state().x(), e.mouse_state().y())
}