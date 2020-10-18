mod world;

use std::ops;

use macroquad::prelude::*;
use macroquad::megaui as ui;

use world::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "The Tenet of Life".to_owned(),
        high_dpi: true,
        window_width: 750,
        window_height: 750,
        ..Default::default()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Vec2i {
    pub x: i32, 
    pub y: i32
}

impl ops::Add<&Vec2i> for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn add(self, _rhs: &Vec2i) -> Vec2i {
        Vec2i { 
            x: self.x + _rhs.x, 
            y: self.y + _rhs.y
        }
    }
}

impl ops::Sub<&Vec2i> for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn sub(self, _rhs: &Vec2i) -> Vec2i {
        Vec2i { 
            x: self.x - _rhs.x, 
            y: self.y - _rhs.y
        }
    }
}

impl ops::Mul<i32> for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn mul(self, _rhs: i32) -> Vec2i {
        Vec2i { 
            x: self.x * _rhs, 
            y: self.y * _rhs
        }
    }
}

impl ops::Mul<f32> for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn mul(self, _rhs: f32) -> Vec2i {
        Vec2i { 
            x: (self.x as f32 * _rhs) as i32, 
            y: (self.y as f32 * _rhs) as i32
        }
    }
}

impl ops::Div<i32> for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn div(self, _rhs: i32) -> Vec2i {
        Vec2i { 
            x: self.x / _rhs, 
            y: self.y / _rhs
        }
    }
}


impl ops::AddAssign for Vec2i {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::SubAssign for Vec2i {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Vec2i {
    #[inline]
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }

    pub fn len(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn is_empty(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Default for Vec2i {
    #[inline]
    fn default() -> Self {
        Vec2i::new(0, 0)
    }
}

impl From<(i32, i32)> for Vec2i {
    #[inline]
    fn from(val: (i32, i32)) -> Self {
        Vec2i::new(val.0, val.1)
    }
}

impl From<(usize, usize)> for Vec2i {
    #[inline]
    fn from(val: (usize, usize)) -> Self {
        Vec2i::new(val.0 as i32, val.1 as i32)
    }
}

impl From<(f32, f32)> for Vec2i {
    #[inline]
    fn from(val: (f32, f32)) -> Self {
        Vec2i::new(val.0 as i32, val.1 as i32)
    }
}

pub fn next_in_rect(pos: &Vec2i, size: &Vec2i) -> Option<Vec2i> {
    if pos.x + 1 != size.x {
        return Some(Vec2i::new(pos.x + 1, pos.y))
    }
    
    if pos.y + 1 != size.y {
        Some(Vec2i::new(0, pos.y + 1))
    } else {
        None
    }
}

#[derive(Clone)]
pub struct FloatImageCamera {
    pub offset: Vec2i,
    pub scale: f32,
}

impl FloatImageCamera {
    pub fn to(&self, pos: Vec2i) -> Vec2i {
        (pos - &self.offset) * self.scale
    }

    pub fn from(&self, pos: Vec2i) -> Vec2i {
        pos * self.scale + &self.offset
    }

    #[allow(dead_code)]
    pub fn from_dir(&self, dir: Vec2i) -> Vec2i {
        dir * self.scale
    }

    pub fn from_i(&self, pos: Vec2i) -> Vec2i {
        pos * (self.scale as i32) + &self.offset
    }

    pub fn from_dir_i(&self, dir: Vec2i) -> Vec2i {
        dir * (self.scale as i32)
    }

    pub fn offset(&mut self, offset: &Vec2i) {
        self.offset += offset.clone();
    }

    pub fn scale_new(&mut self, mouse_pos: &Vec2i, new_scale: f32) {
        self.offset = (self.offset.clone() - mouse_pos) * (new_scale / self.scale) + mouse_pos;
        self.scale = new_scale;
    }

    pub fn scale_add(&mut self, mouse_pos: &Vec2i, add_to_scale: f32) {
        if self.scale + add_to_scale <= 0.0 { return; }
        if self.scale + add_to_scale > 256.0 { return; }

        self.scale_new(mouse_pos, self.scale + add_to_scale);
    }

    pub fn scale_mul(&mut self, mouse_pos: &Vec2i, mul_to_scale: f32) {
        if self.scale * mul_to_scale > 256.0 { return; }

        self.scale_new(mouse_pos, self.scale * mul_to_scale);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale as f32
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = 300;
    let h = 300;

	let mut world = World::new(TheTenetOfLife::calculate().unwrap(), w/2, h/2);

    let mut buffer = vec![WHITE; w * h];
    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture = load_texture_from_image(&image);
    texture.set_filter(unsafe { get_internal_gl().quad_context }, FilterMode::Nearest);

    let mut buffer2 = vec![WHITE; w * h];
    let mut image2 = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture2 = load_texture_from_image(&image);
    texture2.set_filter(unsafe { get_internal_gl().quad_context }, FilterMode::Nearest);

    let mut size = 3usize;
    let mut i = 0i64;
    let mut to_zero = world.clone();
    let mut show_zero_step = false;

    let mut cam = FloatImageCamera {
        offset: Vec2i::new(150, 150),
        scale: 1.5,
    };
    let mut last_mouse_pos = Vec2i::new(mouse_position().0 as i32, mouse_position().1 as i32);
    let mut mouse_move = false;

    loop {
        clear_background(GRAY);

        if i.abs() >= 100 {
            show_zero_step = false;
        }

        let mouse_raw = Vec2i::new(mouse_position().0 as i32, mouse_position().1 as i32);
        let mut mouse = (mouse_raw.clone() - &cam.offset) * (1.0 / cam.scale);
        mouse.x = world::normalize(mouse.x as usize, w) as i32;
        mouse.y = world::normalize(mouse.y as usize, h) as i32;

        let (_, mouse_wheel_y) = mouse_wheel();

        if show_zero_step {
            to_zero.arr_mut().iter_mut().zip(world.arr().iter()).for_each(|(to_zero, world)| *to_zero = *world);
            let mut i_to_zero = i;
            while i_to_zero != 0 {
                if i.signum() == -1 {
                    to_zero.step();
                } else {
                    to_zero.step_back();
                }
                i_to_zero -= i.signum();
            }

            buffer2.iter_mut().zip(to_zero.arr().iter()).for_each(|(buffer, &world)| {
                *buffer = match world as u8 {
                    0 => BLACK,
                    1 => BLUE,
                    2 => RED,
                    _ => unreachable!(),
                };
            });

            for x in 0..size as usize {
                for y in 0..size as usize {
                    if let Some(x) = buffer2.get_mut((mouse.x as usize + x) + (mouse.y as usize + y) * w) {
                        *x = YELLOW;
                    }
                }
            }

            image2.update(&buffer2);
            update_texture(texture2, &image2);
            draw_texture_ex(texture2, cam.offset.x as f32, cam.offset.y as f32 + (h as f32 + 10.) * cam.scale, WHITE, DrawTextureParams { 
                dest_size: Some(Vec2::new(w as f32 * cam.scale, h as f32 * cam.scale)),
                source: None,
                rotation: 0.,
                pivot: None,
            });
        }

        buffer.iter_mut().zip(world.arr().iter()).for_each(|(buffer, &world)| {
            *buffer = match world as u8 {
                0 => BLACK,
                1 => BLUE,
                2 => RED,
                _ => unreachable!(),
            };
        });

        for x in 0..size as usize {
            for y in 0..size as usize {
                if let Some(x) = buffer.get_mut((mouse.x as usize + x) + (mouse.y as usize + y) * w) {
                    *x = YELLOW;
                }
            }
        }

        image.update(&buffer);
        update_texture(texture, &image);
        draw_texture_ex(texture, cam.offset.x as f32, cam.offset.y as f32, WHITE, DrawTextureParams { 
            dest_size: Some(Vec2::new(w as f32 * cam.scale, h as f32 * cam.scale)),
            source: None,
            rotation: 0.,
            pivot: None,
        });

        let mut mouse_over_canvas = true;
        draw_window(
            hash!(),
            vec2(10., 10.),
            vec2(270., 310.),
            WindowParams {
                label: "Controls".to_string(),
                close_button: false,
                ..Default::default()
            },
            |ui| {
                mouse_over_canvas &= !ui.is_mouse_over(ui::Vector2::new(mouse_position().0, mouse_position().1));
                {
                    ui.label(None, &format!(" Mouse position on canvas: ({}, {})", mouse.x, mouse.y));
                }
                {
                    ui.label(None, " Step: ");
                    ui.same_line(0.0);
                    if ui.button(None, "-10") {
                        for _ in 0..10 { world.step_back(); }
                        i -= 10;
                    }
                    ui.same_line(0.0);
                    if ui.button(None, "-") {
                        world.step_back();
                        i -= 1;
                    }
                    ui.same_line(0.0);
                    ui.label(None, &format!("{:5}", i));
                    ui.same_line(0.0);
                    if ui.button(None, "+") {
                        world.step();
                        i += 1;
                    }
                    ui.same_line(0.0);
                    if ui.button(None, "+10") {
                        for _ in 0..10 { world.step(); }
                        i += 10;
                    }    
                }
                {
                    ui.label(None, " Draw size: ");
                    ui.same_line(0.0);
                    if ui.button(None, "-") {
                        size = size.saturating_sub(1);
                    }
                    ui.same_line(0.0);
                    ui.label(None, &format!("{:2}", size));
                    ui.same_line(0.0);
                    if ui.button(None, "+") {
                        size = size.saturating_add(1);
                    }
                }
                ui.separator();
                {
                    ui.label(None, " Zero step showed only");
                    ui.label(None, " for |step| < 100.");
                    ui.label(None, " Show zero step: ");
                    ui.same_line(0.0);
                    if ui.button(None, if show_zero_step { "Yes" } else { "No" }) {
                        show_zero_step = !show_zero_step;
                    }   
                }
                ui.separator();
                {
                    ui.label(None, " Mouse control:");
                    ui.label(None, "  Left button - draw blue cells");
                    ui.label(None, "  Right button - draw red cells");
                    ui.label(None, "  Middle button - move image");
                    ui.label(None, "  Left + Right button - clear");
                    ui.label(None, "  Shift + Wheel - change draw size");
                    ui.label(None, "  Ctrl + Wheel - simulate");
                }
            },
        );

        if is_key_down(KeyCode::LeftShift) {
            if mouse_wheel_y > 0. {
                size = size.saturating_add(1);
            } else if mouse_wheel_y < 0. {
                size = size.saturating_sub(1);
            }
        } else if is_key_down(KeyCode::LeftControl) {
            if mouse_wheel_y > 0. {
                world.step();
                i += 1;
            } else if mouse_wheel_y < 0. {
                world.step_back();
                i -= 1;
            }
        }

        if mouse_over_canvas {
            if is_mouse_button_down(MouseButton::Left) && is_mouse_button_down(MouseButton::Right) {
                world.set_rect(mouse.x as usize, mouse.y as usize, size, size, 0);
            } else if is_mouse_button_down(MouseButton::Left) {
                world.set_rect(mouse.x as usize, mouse.y as usize, size, size, 1);
            } else if is_mouse_button_down(MouseButton::Right) {
                world.set_rect(mouse.x as usize, mouse.y as usize, size, size, 2);
            }


            if !is_key_down(KeyCode::LeftShift) && !is_key_down(KeyCode::LeftControl) {
                if mouse_wheel_y > 0. {
                    cam.scale_mul(&last_mouse_pos, 1.2);
                } else if mouse_wheel_y < 0. {
                    cam.scale_mul(&last_mouse_pos, 1.0 / 1.2);
                }
            }
            

            mouse_move = is_mouse_button_down(MouseButton::Middle);
            
        }
        if mouse_move {
            cam.offset(&(mouse_raw.clone() - &last_mouse_pos));
        }
        last_mouse_pos = mouse_raw;

        next_frame().await
    }
}