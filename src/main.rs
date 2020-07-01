extern crate sdl2; 

mod particle_plane;
mod particle_behaviour;
mod finals;
mod gui;

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
    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_SIZE.0 as u32 + 100, WINDOW_SIZE.1 as u32 + 100)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut start = Instant::now();

    let mut pplane = ParticlePlane::new();
    let mut current_draw_particle_index: u8 = 1;
    'running: loop {
        //HANDLE MOUSE INPUT
        let mouse_x = get_cursor_pos(&event_pump).0;
        let mouse_y = get_cursor_pos(&event_pump).1;

        //CHECK IF MOUSE INSIDE PARTICLE PLANE
        if mouse_x < WINDOW_SIZE.0 as i32 && mouse_y < WINDOW_SIZE.1 as i32 {
            if event_pump.mouse_state().left() {
                let x = get_cursor_pos(&event_pump).0;
                let y = get_cursor_pos(&event_pump).1;
                if pplane.grid[(x/GRID_SLOT_SIZE.0) as usize][(y/GRID_SLOT_SIZE.1) as usize].is_none() {
                    pplane.grid[(x/GRID_SLOT_SIZE.0) as usize][(y/GRID_SLOT_SIZE.1) as usize] = Some(get_draw_particle(current_draw_particle_index));
                }
            }
        }
        if mouse_x < WINDOW_SIZE.0 as i32 && mouse_y < WINDOW_SIZE.1 as i32 {
            if event_pump.mouse_state().right() {
                let x = get_cursor_pos(&event_pump).0;
                let y = get_cursor_pos(&event_pump).1;
                pplane.grid[(x/GRID_SLOT_SIZE.0) as usize][(y/GRID_SLOT_SIZE.1) as usize] = Some(get_draw_particle(current_draw_particle_index));
            }
        }
        

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Tab), ..} => {
                    current_draw_particle_index += 1;
                    if current_draw_particle_index > 12 { current_draw_particle_index = 1 };
                }
                _ => {}
            }
        }
        if start.elapsed().as_millis() > 18 {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            draw_plane_border(&mut canvas);

            // CODE
            pplane.reset_updateable();
            pplane.update();
            pplane.draw(&mut canvas);

            display_current_draw_particle(&mut canvas, 30, 960, current_draw_particle_index);
            canvas.present();
            start = Instant::now();
        }
        
        
    }
    Ok(())
}

fn get_cursor_pos(e: &sdl2::EventPump) -> (i32, i32){
    (e.mouse_state().x(), e.mouse_state().y())
}


fn draw_plane_border<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>){
    #![allow(warnings)]
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(sdl2::rect::Rect::new(WINDOW_SIZE.0 as i32, 0, 2, WINDOW_SIZE.1 as u32 + 2));
    canvas.fill_rect(sdl2::rect::Rect::new(0, WINDOW_SIZE.1 as i32, WINDOW_SIZE.0 as u32 + 1, 2));
}

fn get_draw_particle(index: u8) -> Particle {
    return match index {
        1 => Particle::new(ParticleType::Sand),
        2 => Particle::new(ParticleType::Glass),
        3 => Particle::new(ParticleType::MoltenGlass),
        4 => Particle::new(ParticleType::Ice),
        5 => Particle::new(ParticleType::Water),
        6 => Particle::new(ParticleType::Steam),
        7 => Particle::new(ParticleType::Stone),
        8 => Particle::new(ParticleType::Lava),
        9 => Particle::new(ParticleType::VaporisedSilicon),
        10 => Particle::new(ParticleType::Osmium),
        11 => Particle::new(ParticleType::MoltenOsmium),
        12 => Particle::new(ParticleType::VaporisedOsmium),
        _ => Particle::new(ParticleType::Glass),
    }
}

pub fn display_current_draw_particle<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, index: u8){
    match index {
        1 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("SAND")),
        2 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("GLASS")),
        3 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("MOLTEN GLASS")),
        4 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("ICE")),
        5 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("WATER")),
        6 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("STEAM")),
        7 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("STONE")),
        8 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("LAVA")),
        9 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("VAPORISED SILICON")),
        10 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("OSMIUM")),
        11 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("MOLTEN OSMIUM")),
        12 => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("VAPORISED OSMIUM")),
        _ => gui::draw_text(canvas, x, y, (255, 255, 255), String::from("GLASS")),
    }
}