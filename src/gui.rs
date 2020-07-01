#![allow(warnings)]

use sdl2::*;



pub fn draw_text<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8), text: String){
    for (i, c) in text.chars().enumerate() {
        match c {
            'A' | 'a' => draw_char_A(canvas, x + i as i32 * 25, y, color),
            'B' | 'b' => draw_char_B(canvas, x + i as i32 * 25, y, color),
            'C' | 'c' => draw_char_C(canvas, x + i as i32 * 25, y, color),
            'D' | 'd' => draw_char_D(canvas, x + i as i32 * 25, y, color),
            'E' | 'e' => draw_char_E(canvas, x + i as i32 * 25, y, color),
            'F' | 'f' => draw_char_F(canvas, x + i as i32 * 25, y, color),
            'G' | 'g' => draw_char_G(canvas, x + i as i32 * 25, y, color),
            'H' | 'h' => draw_char_H(canvas, x + i as i32 * 25, y, color),
            'I' | 'i' => draw_char_I(canvas, x + i as i32 * 25, y, color),
            'J' | 'j' => draw_char_J(canvas, x + i as i32 * 25, y, color),
            'K' | 'k' => draw_char_K(canvas, x + i as i32 * 25, y, color),
            'L' | 'l' => draw_char_L(canvas, x + i as i32 * 25, y, color),
            'M' | 'm' => draw_char_M(canvas, x + i as i32 * 25, y, color),
            'N' | 'n' => draw_char_N(canvas, x + i as i32 * 25, y, color),
            'O' | 'o' => draw_char_O(canvas, x + i as i32 * 25, y, color),
            'P' | 'p' => draw_char_P(canvas, x + i as i32 * 25, y, color),
            'Q' | 'q' => draw_char_Q(canvas, x + i as i32 * 25, y, color),
            'R' | 'r' => draw_char_R(canvas, x + i as i32 * 25, y, color),
            'S' | 's' => draw_char_S(canvas, x + i as i32 * 25, y, color),
            'T' | 't' => draw_char_T(canvas, x + i as i32 * 25, y, color),
            'U' | 'u' => draw_char_U(canvas, x + i as i32 * 25, y, color),
            'V' | 'v' => draw_char_V(canvas, x + i as i32 * 25, y, color),
            'W' | 'w' => draw_char_W(canvas, x + i as i32 * 25, y, color),
            'X' | 'x' => draw_char_X(canvas, x + i as i32 * 25, y, color),
            'Y' | 'y' => draw_char_Y(canvas, x + i as i32 * 25, y, color),
            'Z' | 'z' => draw_char_Z(canvas, x + i as i32 * 25, y, color),
            _ => draw_char_X(canvas, x + i as i32 * 25, y, color),
        }
    }
}

pub fn draw_char_A<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x + 10, y - 20));
    canvas.draw_line((x + 10, y - 20), (x + 20, y));
    canvas.draw_line((x - 4, y - 10), (x + 24, y - 10));
}

pub fn draw_char_B<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x + 10, y - 20), (x + 10, y - 10));
    canvas.draw_line((x + 10, y - 10), (x, y - 10));
    canvas.draw_line((x + 10, y - 10), (x + 10, y));
    canvas.draw_line((x + 10, y), (x, y));
}

pub fn draw_char_C<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x, y), (x + 10, y));
}

pub fn draw_char_D<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 17));
    canvas.draw_line((x + 10, y - 17), (x + 10, y - 3));
    canvas.draw_line((x + 10, y - 3), (x, y));
}

pub fn draw_char_E<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y - 10));
    canvas.draw_line((x, y), (x + 10, y));
}

pub fn draw_char_F<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y - 10));
}

pub fn draw_char_G<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x, y), (x + 10, y));
    canvas.draw_line((x + 10, y), (x + 10, y - 10));
    canvas.draw_line((x + 6, y - 10), (x + 10, y - 10));
}

pub fn draw_char_H<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x + 10, y), (x + 10, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y - 10));
}

pub fn draw_char_I<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
}

pub fn draw_char_J<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x + 10, y), (x + 10, y - 20));
    canvas.draw_line((x + 5, y), (x + 10, y));
}

pub fn draw_char_K<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y));
}

pub fn draw_char_L<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y), (x + 10, y));
}

pub fn draw_char_M<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x + 5, y - 20));
    canvas.draw_line((x + 5, y - 20), (x + 10, y));
    canvas.draw_line((x + 10, y), (x + 15, y - 20));
    canvas.draw_line((x + 15, y - 20), (x + 20, y));
}

pub fn draw_char_N<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x + 5, y - 20));
    canvas.draw_line((x + 5, y - 20), (x + 10, y));
    canvas.draw_line((x + 10, y), (x + 15, y - 20));
}

pub fn draw_char_O<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x + 10, y - 20), (x + 10, y - 10));
    canvas.draw_line((x + 10, y - 10), (x + 10, y));
    canvas.draw_line((x + 10, y), (x, y));
}

pub fn draw_char_P<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y - 10));
    canvas.draw_line((x + 10, y - 10), (x + 10, y - 20));
}

pub fn draw_char_Q<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x + 10, y - 20), (x + 10, y - 10));
    canvas.draw_line((x + 10, y - 10), (x + 10, y));
    canvas.draw_line((x + 10, y), (x, y));
    canvas.draw_line((x + 8, y - 3), (x + 12, y + 3));
}

pub fn draw_char_R<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y - 10));
    canvas.draw_line((x + 10, y - 10), (x + 10, y - 20));
    canvas.draw_line((x, y - 10), (x + 10, y));
}

pub fn draw_char_S<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x + 10, y));
    canvas.draw_line((x + 10, y), (x + 10, y - 10));
    canvas.draw_line((x + 10, y - 10), (x, y - 10));
    canvas.draw_line((x, y - 10), (x, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
}

pub fn draw_char_T<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x + 5, y), (x + 5, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
}

pub fn draw_char_U<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x, y - 20));
    canvas.draw_line((x + 10, y), (x + 10, y - 20));
    canvas.draw_line((x, y), (x + 10, y));
}

pub fn draw_char_V<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y - 20), (x + 5, y));
    canvas.draw_line((x + 5, y), (x + 10, y - 20));
}

pub fn draw_char_W<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y - 20), (x + 5, y));
    canvas.draw_line((x + 5, y), (x + 10, y - 20));
    canvas.draw_line((x + 10, y - 20), (x + 15, y));
    canvas.draw_line((x + 15, y), (x + 20, y - 20));
}

pub fn draw_char_X<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y - 20), (x + 10, y));
    canvas.draw_line((x, y), (x + 10, y - 20));
}

pub fn draw_char_Y<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y - 20), (x + 5, y - 10));
    canvas.draw_line((x + 5, y - 10), (x + 10, y - 20));
    canvas.draw_line((x + 5, y - 10), (x + 5, y));
}

pub fn draw_char_Z<T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, x: i32, y: i32, color: (u8, u8, u8)){
    canvas.set_draw_color(color);
    canvas.draw_line((x, y), (x + 10, y));
    canvas.draw_line((x, y), (x + 10, y - 20));
    canvas.draw_line((x, y - 20), (x + 10, y - 20));
}