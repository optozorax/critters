mod world;
mod rules;

use std::convert::TryFrom;
use std::convert::TryInto;
use crate::rules::{Rules, RulesTwoStates, RulesThreeStates, BlockInt};
use permutation_string::PermutationArray;

use thiserror::Error;

use std::ops;

use macroquad::prelude::*;
use macroquad::megaui as ui;

use world::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Invertible automata simulation".to_owned(),
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

pub struct ChangeRulesWindow {
    current_rules_type: usize,
    draw_self: bool,
    is_changed: bool,
    current_rules: Vec<PermutationArray>,
    current_rules_name: String,
    known_rules: Vec<(&'static str, Vec<(&'static str, Vec<PermutationArray>)>)>,
    label: Option<String>,
}

impl Default for ChangeRulesWindow {
    fn default() -> Self {
        let known_rules = vec![
            ("2 states, 1 step",
            vec![
                ("Critters", vec![PermutationArray(vec![
                    0b1111, 0b1110, 0b1101, 0b0011,
                    0b1011, 0b0101, 0b0110, 0b0001,
                    0b0111, 0b1001, 0b1010, 0b0010,
                    0b1100, 0b0100, 0b1000, 0b0000,
                ])]), 
                ("Billiard Ball Machine", vec![PermutationArray(vec![
                    0b0000, 0b1000, 0b0100, 0b0011,
                    0b0010, 0b0101, 0b1001, 0b0111,
                    0b0001, 0b0110, 0b1010, 0b1011,
                    0b1100, 0b1101, 0b1110, 0b1111,
                ])]), 
                ("Single Rotate", vec![PermutationArray(vec![
                    0,2,8,3,1,5,6,7,4,9,10,11,12,13,14,15
                ])]), 
                ("Bounce gas", vec![PermutationArray(vec![
                    0,8,4,3,2,5,9,14,1,6,10,13,12,11,7,15
                ])]), 
                ("HPP gas", vec![PermutationArray(vec![
                    0,8,4,12,2,10,9,14,1,6,5,13,3,11,7,15
                ])]), 
                ("Rotations", vec![PermutationArray(vec![
                    0,2,8,12,1,10,9,11,4,6,5,14,3,7,13,15
                ])]), 
                ("Rotations 2", vec![PermutationArray(vec![
                    0,2,8,12,1,10,9,13,4,6,5,7,3,14,11,15
                ])]), 
                ("Rotations 3", vec![PermutationArray(vec![
                    0,4,1,10,8,3,9,11,2,6,12,14,5,7,13,15
                ])]), 
                ("Rotations 4", vec![PermutationArray(vec![
                    0,4,1,12,8,10,6,14,2,9,5,13,3,11,7,15
                ])]), 
                ("String Thing", vec![PermutationArray(vec![
                    0,1,2,12,4,10,9,7,8,6,5,11,3,13,14,15
                ])]), 
                ("String Thing 2", vec![PermutationArray(vec![
                    0,1,2,12,4,10,6,7,8,9,5,11,3,13,14,15
                ])]), 
                ("Swap On Diag", vec![PermutationArray(vec![
                    0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15
                ])]), 
                ("Tron", vec![PermutationArray(vec![
                    15,1,2,3,4,5,6,7,8,9,10,11,12,13,14,0
                ])]), 
                ("Double Rotate", vec![PermutationArray(vec![
                    0,2,8,3,1,5,6,13,4,9,10,7,12,14,11,15
                ])]),
            ]),
            ("2 states, 2 step",
            vec![
                (
                    "Critters with inverted 2 step",
                    vec![
                        PermutationArray(vec![
                            0b1111, 0b1110, 0b1101, 0b0011,
                            0b1011, 0b0101, 0b0110, 0b0001,
                            0b0111, 0b1001, 0b1010, 0b0010,
                            0b1100, 0b0100, 0b1000, 0b0000,
                        ]),
                        // TODO
                        PermutationArray(vec![
                            0b1111, 0b1110, 0b1101, 0b0011,
                            0b1011, 0b0101, 0b0110, 0b0001,
                            0b0111, 0b1001, 0b1010, 0b0010,
                            0b1100, 0b0100, 0b1000, 0b0000,
                        ])
                    ]
                ),
            ]),
            ("3 states, 1 step", vec![]),
            ("3 states, 2 step",
            vec![
                (
                    "The Tenet Of Life",
                    vec![
                        // TODO вычислять это на этапе запуска
                        PermutationArray(vec![
                            0,  1,  54, 3,  36, 7,  18, 5,  72, 
                            9,  30, 19, 28, 39, 66, 21, 48, 75, 
                            6,  11, 60, 15, 42, 69, 56, 51, 26, 
                            27, 12, 55, 10, 37, 64, 57, 46, 73, 
                            4,  31, 58, 13, 40, 67, 22, 49, 76, 
                            63, 34, 61, 16, 43, 70, 25, 68, 79, 
                            2,  29, 24, 33, 38, 65, 20, 47, 62, 
                            45, 32, 59, 14, 41, 52, 23, 50, 77, 
                            8 , 35, 74, 17, 44, 71, 78, 53, 80,
                        ]),
                        PermutationArray(vec![
                            0,  27, 2,  9,  36, 7,  6,  5,  72, 
                            3,  30, 19, 28, 13, 66, 21, 48, 75, 
                            18, 11, 60, 15, 42, 69, 56, 51, 78, 
                            1,  12, 55, 10, 31, 64, 57, 46, 73, 
                            4,  37, 58, 39, 40, 67, 22, 49, 76, 
                            63, 34, 61, 16, 43, 70, 25, 68, 79, 
                            54, 29, 24, 33, 38, 65, 20, 47, 74, 
                            45, 32, 59, 14, 41, 52, 23, 50, 77, 
                            8,  35, 62, 17, 44, 71, 26, 53, 80,
                        ])
                    ]
                ),
            ]),
        ];
        let current_rules_type = 1;
        let (current_rules_name, current_rules) = known_rules[current_rules_type].1[0].clone();
        ChangeRulesWindow {
            draw_self: false,
            is_changed: true,
            current_rules_type,
            current_rules,
            current_rules_name: current_rules_name.to_string(),
            known_rules,
            label: None,
        }
    }
}

impl ChangeRulesWindow {
    pub fn get_default_rules(&self) -> Box<dyn Rules> {
        macro_rules! block_arr { ($($a:expr),+ $(,)?) => { [$(BlockInt($a)),+] }; }

        let the_tenet_of_life_step1 = block_arr![
            0,  1,  54, 3,  36, 7,  18, 5,  72, 
            9,  30, 19, 28, 39, 66, 21, 48, 75, 
            6,  11, 60, 15, 42, 69, 56, 51, 26, 
            27, 12, 55, 10, 37, 64, 57, 46, 73, 
            4,  31, 58, 13, 40, 67, 22, 49, 76, 
            63, 34, 61, 16, 43, 70, 25, 68, 79, 
            2,  29, 24, 33, 38, 65, 20, 47, 62, 
            45, 32, 59, 14, 41, 52, 23, 50, 77, 
            8 , 35, 74, 17, 44, 71, 78, 53, 80,
        ];

        let the_tenet_of_life_step2 = block_arr![
            0,  27, 2,  9,  36, 7,  6,  5,  72, 
            3,  30, 19, 28, 13, 66, 21, 48, 75, 
            18, 11, 60, 15, 42, 69, 56, 51, 78, 
            1,  12, 55, 10, 31, 64, 57, 46, 73, 
            4,  37, 58, 39, 40, 67, 22, 49, 76, 
            63, 34, 61, 16, 43, 70, 25, 68, 79, 
            54, 29, 24, 33, 38, 65, 20, 47, 74, 
            45, 32, 59, 14, 41, 52, 23, 50, 77, 
            8,  35, 62, 17, 44, 71, 26, 53, 80,
        ];

        Box::new(RulesThreeStates::from_two_steps(the_tenet_of_life_step1, the_tenet_of_life_step2).unwrap())
    }

    pub fn draw_window_for_change_rules(&mut self, mouse_over_canvas: &mut bool) -> Option<Box<dyn Rules>> {
        let mut to_return = None;
        draw_window(
            hash!(),
            vec2(10., 10.),
            vec2(270., 510.),
            WindowParams {
                label: "Change rules".to_string(),
                close_button: true,
                ..Default::default()
            },
            |ui| {
                *mouse_over_canvas &= !ui.is_mouse_over(ui::Vector2::new(mouse_position().0, mouse_position().1));
                let mut change_rules = None;
                ui.tree_node(hash!(), "Known rules", |ui| {
                    for (name, rules) in &self.known_rules {
                        ui.label(None, name);
                        ui.separator();
                        for (rule_name, rule_array) in rules.iter() {
                            if ui.button(None, rule_name) {
                                change_rules = Some((rule_array.clone(), rule_name));
                            }
                        }
                    }
                });
                if let Some((rules, name)) = change_rules {
                    match construct_rules(rules) {
                        Ok(rules) => {
                            self.label = Some(format!("Successfully changed rules to {}", name));
                            to_return = Some(rules);
                        },
                        Err(err) => {
                            self.label = Some(err.to_string())
                        }
                    }
                }
                if let Some(label) = self.label.as_ref() {
                    ui.label(None, label);
                }
            }
        );
        to_return
    }

    pub fn activate(&mut self) {
        self.draw_self = true;
    }
}

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Error)]
pub enum ConstructError {
    #[error("wrong count of steps: `{steps_count}`, supported only 1 or 2 steps")]
    WrongSteps { steps_count: usize },
    #[error("wrong elements count in array")]
    WrongElementsCount,
    #[error("arrays is not permutation arrays")]
    NotPermutationArrays,
}

pub fn construct_rules(permutation_vecs: Vec<PermutationArray>) -> Result<Box<dyn Rules>, ConstructError> {
    macro_rules! to_array {
        ($size:tt, $array:ident) => {
            <[BlockInt; $size]>::try_from(
                &$array.0
                .into_iter()
                .map(|x| x.try_into().ok().map(BlockInt))
                .collect::<Option<Vec<BlockInt>>>().ok_or(ConstructError::NotPermutationArrays)?[..]
            ).unwrap()
        };
    }
    let mut it = permutation_vecs.into_iter();
    match (it.next(), it.next(), it.next()) {
        (Some(a), None, None) => {
            if a.0.len() == 16 {
                Ok(Box::new(RulesTwoStates::from_one_step(to_array!(16, a)).ok_or(ConstructError::NotPermutationArrays)?))
            } else if a.0.len() == 81 {
                Ok(Box::new(RulesThreeStates::from_one_step(to_array!(81, a)).ok_or(ConstructError::NotPermutationArrays)?))
            } else {
                Err(ConstructError::WrongElementsCount)
            }
        },
        (Some(a), Some(b), None) => {
            if a.0.len() == 16 && b.0.len() == 16 {
                Ok(Box::new(RulesTwoStates::from_two_steps(to_array!(16, a), to_array!(16, b)).ok_or(ConstructError::NotPermutationArrays)?))
            } else if a.0.len() == 81 && b.0.len() == 81 {
                Ok(Box::new(RulesThreeStates::from_two_steps(to_array!(81, a), to_array!(81, b)).ok_or(ConstructError::NotPermutationArrays)?))
            } else {
                Err(ConstructError::WrongElementsCount)
            }
        },
        _ => Err(ConstructError::WrongSteps { steps_count: 2 + it.count() }),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = 100;
    let h = 100;

    let mut rules_window = ChangeRulesWindow::default();

	let tenet_world = World::new(rules_window.get_default_rules(), w/2, h/2);

    let mut world = tenet_world;

    let mut buffer = vec![WHITE; w * h];
    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture = load_texture_from_image(&image);
    set_texture_filter(texture, FilterMode::Nearest);

    let mut size = 3usize;
    let mut i = 0i64;

    let mut cam = FloatImageCamera {
        offset: Vec2i::new(150, 150),
        scale: 1.5 * 300.0 / w as f32,
    };
    let mut last_mouse_pos = Vec2i::new(mouse_position().0 as i32, mouse_position().1 as i32);
    let mut mouse_move = false;
    let mut clear_mouse = 0;

    let mut draw_grid = false;

    let cell_size = 10;
    let main_border = 2;
    let secondary_border = 1;
    let full_size = main_border + cell_size + secondary_border + cell_size;

    let (new_w, new_h) = ((w/2) as u32 * full_size + main_border, (h/2) as u32 * full_size + main_border);
    let render_target = render_target(new_w, new_h);
    set_texture_filter(render_target.texture, FilterMode::Nearest);
    let material = load_material(
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![
                ("Size".to_owned(), UniformType::Float2), 
                ("TextureSize".to_owned(), UniformType::Float2),
                ("cell_size".to_owned(), UniformType::Int1),
                ("main_border".to_owned(), UniformType::Int1),
                ("secondary_border".to_owned(), UniformType::Int1),
                ("full_size".to_owned(), UniformType::Int1),
                ("offset".to_owned(), UniformType::Int1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    const FRAGMENT_SHADER: &str = r#"#version 100
    #extension GL_EXT_gpu_shader4 : enable
    precision lowp float;

    varying vec4 color;
    varying vec2 uv;
    varying vec2 intpos;
        
    uniform sampler2D Texture;
    uniform vec2 TextureSize;

    uniform int cell_size;
    uniform int main_border;
    uniform int secondary_border;
    uniform int full_size;
    uniform int offset;

    void calcColor(int pos, vec2 dir, inout vec2 a, vec2 s, inout int change_priority) {
        pos = pos % full_size;
        if (offset == 1) {
            if (pos < main_border) {
                if (change_priority <= 2) {
                    gl_FragColor = vec4(vec3(0.8), 1.0);
                    change_priority = 2;
                }
            } else if (pos < main_border + cell_size) {
                if (change_priority == 0) {
                    gl_FragColor = vec4(texture2D(Texture, a / s).rgb, 1.0);
                    change_priority = 0;
                }
            } else if (pos < main_border + cell_size + secondary_border) {
                if (change_priority <= 1) {
                    gl_FragColor = vec4(vec3(0.4), 1.0);
                    change_priority = 1;
                }
            } else {
                if (change_priority == 0) {
                    a += 1.0 * dir;
                    gl_FragColor = vec4(texture2D(Texture, a / s).rgb, 1.0);
                    change_priority = 0;
                }
            }
        } else {
            if (pos < secondary_border) {
                if (change_priority <= 1) {
                    gl_FragColor = vec4(vec3(0.4), 1.0);
                    change_priority = 1;
                }
            } else if (pos < secondary_border + cell_size) {
                if (change_priority == 0) {
                    gl_FragColor = vec4(texture2D(Texture, a / s).rgb, 1.0);
                    change_priority = 0;
                }
            } else if (pos < secondary_border + cell_size + main_border) {
                if (change_priority <= 2) {
                    gl_FragColor = vec4(vec3(0.8), 1.0);
                    change_priority = 2;
                }
            } else {
                if (change_priority == 0) {
                    a += 1.0 * dir;
                    gl_FragColor = vec4(texture2D(Texture, a / s).rgb, 1.0);
                    change_priority = 0;
                }
            }
        }
    }

    void main() {
        highp vec2 a = intpos;
        highp vec2 s = TextureSize;
        a /= float(full_size);
        a = vec2(float(int(a.x)), float(int(a.y)));
        a *= 2.0;
        a += vec2(1.0, 1.0);

        int change_priority = 0;
        calcColor(int(intpos.x), vec2(1.0, 0.0), a, s, change_priority);
        calcColor(int(intpos.y), vec2(0.0, 1.0), a, s, change_priority);
    }
    "#;

    const VERTEX_SHADER: &str = "#version 100
    attribute vec3 position;
    attribute vec2 texcoord;
    attribute vec4 color0;

    varying lowp vec2 uv;
    varying lowp vec4 color;
    varying lowp vec2 intpos;

    uniform mat4 Model;
    uniform mat4 Projection;
    uniform vec2 Size;

    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        color = color0 / 255.0;
        uv = texcoord;
        intpos = (position.xy + vec2(1.0, 1.0)) / 2.0 * Size;
    }
    ";


    loop {
        clear_background(GRAY);

        let mouse_raw = Vec2i::new(mouse_position().0 as i32, mouse_position().1 as i32);
        let mut mouse = (mouse_raw.clone() - &cam.offset) * (1.0 / cam.scale);
        mouse.x = world::normalize(mouse.x as usize, w) as i32;
        mouse.y = world::normalize(mouse.y as usize, h) as i32;

        let (_, mouse_wheel_y) = mouse_wheel();

        buffer.iter_mut().zip(world.arr().iter()).for_each(|(buffer, &world)| {
            *buffer = match world.0 {
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

        if draw_grid {
            set_camera(Camera2D {
                render_target: Some(render_target),
                ..Default::default()
            });
                material.set_uniform("Size", (new_w as f32, new_h as f32));
                material.set_uniform("TextureSize", (w as f32, h as f32));
                material.set_uniform("cell_size", (cell_size,));
                material.set_uniform("main_border", (main_border,));
                material.set_uniform("secondary_border", (secondary_border,));
                material.set_uniform("full_size", (full_size,));
                material.set_uniform("offset", (!world.is_intermediate_step() as u32,));
                gl_use_material(material);
                    draw_texture_ex(
                        texture,
                        -1.,
                        -1.,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(2.0, 2.0)),
                            ..Default::default()
                        },
                    );
                gl_use_default_material();
            set_default_camera();

            draw_texture_ex(render_target.texture, cam.offset.x as f32, cam.offset.y as f32, WHITE, DrawTextureParams { 
                dest_size: Some(Vec2::new(w as f32 * cam.scale, h as f32 * cam.scale)),
                source: None,
                rotation: 0.,
                pivot: None,
            });
        } else {
            draw_texture_ex(texture, cam.offset.x as f32, cam.offset.y as f32, WHITE, DrawTextureParams { 
                dest_size: Some(Vec2::new(w as f32 * cam.scale, h as f32 * cam.scale)),
                source: None,
                rotation: 0.,
                pivot: None,
            });
        }

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
                    if ui.button(None, "Change rules") {
                        rules_window.activate();
                    }
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
                {
                    ui.label(None, " Draw grid: ");
                    ui.same_line(0.0);
                    if ui.button(None, if draw_grid { "Yes" } else { "No" }) {
                        draw_grid = !draw_grid;
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

        if let Some(new_rules) = rules_window.draw_window_for_change_rules(&mut mouse_over_canvas) {
            world.change_rules(new_rules);
        }

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
                world.set_rect(mouse.x as usize, mouse.y as usize, size, size, world.get_rules().mouse_3());
                clear_mouse = 0b11;
            } else if is_mouse_button_down(MouseButton::Left) && clear_mouse == 0 {
                world.set_rect(mouse.x as usize, mouse.y as usize, size, size, world.get_rules().mouse_1());
            } else if is_mouse_button_down(MouseButton::Right) && clear_mouse == 0 {
                world.set_rect(mouse.x as usize, mouse.y as usize, size, size, world.get_rules().mouse_2());
            }

            if !is_mouse_button_down(MouseButton::Left) {
                clear_mouse &= 0b01;
            }
            if !is_mouse_button_down(MouseButton::Right) {
                clear_mouse &= 0b10;
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