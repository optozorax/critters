mod world;

use macroquad::*;

use world::*;

#[macroquad::main("Life")]
async fn main() {
    let w = 300;
    let h = 300;

	let mut world = World::new(w/2, h/2, false);

    let mut buffer = vec![WHITE; w * h];
    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture = load_texture_from_image(&image);

    let mut buffer2 = vec![WHITE; w * h];
    let mut image2 = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture2 = load_texture_from_image(&image);

    let mut size = 3u8;

    let mut i = 0i64;

    let mut to_zero = world.clone();

    let mut show_zero_step = false;

    loop {
        clear_background(BLUE);

        if is_key_down(KeyCode::Up) {
            size = size.saturating_add(1);
        }
        if is_key_down(KeyCode::Down) {
            size = size.saturating_sub(1);
        }

        if is_key_down(KeyCode::Right) {
            world.step();
            world.step();

            i += 1;
        }
        if is_key_down(KeyCode::Left) {
            world.step_back();
            world.step_back();

            i -= 1;
        }

        if is_key_down(KeyCode::B) && i.abs() < 100 {
            show_zero_step = !show_zero_step;
        }

        if i.abs() >= 100 {
            show_zero_step = false;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let mouse_x = world::normalize(mouse_x as usize, w);
        let mouse_y = world::normalize(mouse_y as usize, h);

        if is_mouse_button_down(MouseButton::Left) {
            for x in 0..size as usize {
                for y in 0..size as usize {
                    world.set(mouse_x + x, mouse_y + y, true);
                }
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            for x in 0..size as usize {
                for y in 0..size as usize {
                    world.set(mouse_x + x, mouse_y + y, false);
                }
            }
        }

        if show_zero_step {
            to_zero.arr.iter_mut().zip(world.arr.iter()).for_each(|(to_zero, world)| *to_zero = *world);
            let mut i_to_zero = i;
            while i_to_zero != 0 {
                if i.signum() == -1 {
                    to_zero.step();
                    to_zero.step();
                } else {
                    to_zero.step_back();
                    to_zero.step_back();
                }
                i_to_zero -= i.signum();
            }

            buffer2.iter_mut().zip(to_zero.arr.iter()).for_each(|(buffer, &world)| {
                *buffer = if world { WHITE } else { BLACK };
            });

            for x in 0..size as usize {
                for y in 0..size as usize {
                    if let Some(x) = buffer2.get_mut((mouse_x + x) + (mouse_y + y) * w) {
                        *x = YELLOW;
                    }
                }
            }

            image2.update(&buffer2);
            update_texture(texture2, &image2);
            draw_texture(texture2, 0., h as f32 + 2., WHITE);

            draw_text(
                "Step: 0",
                w as f32 + 10.0,
                h as f32 + 10.0,
                20.,
                BLACK,
            );
        }

        buffer.iter_mut().zip(world.arr.iter()).for_each(|(buffer, &world)| {
            *buffer = if world { WHITE } else { BLACK };
        });

        for x in 0..size as usize {
            for y in 0..size as usize {
                if let Some(x) = buffer.get_mut((mouse_x + x) + (mouse_y + y) * w) {
                    *x = YELLOW;
                }
            }
        }

        image.update(&buffer);
        update_texture(texture, &image);
        draw_texture(texture, 0., 0., WHITE);

        draw_text(
            format!("Step: {}", i).as_str(),
            w as f32 + 10.0,
            10.0,
            20.,
            BLACK,
        );
        draw_text(
            format!("Draw size: {}", size).as_str(),
            w as f32 + 10.0,
            30.0,
            20.,
            BLACK,
        );
        draw_text(
            format!("Show zero step: {}", show_zero_step).as_str(),
            w as f32 + 10.0,
            50.0,
            20.,
            BLACK,
        );
         draw_text(
            "Works only for |step| < 100",
            w as f32 + 10.0,
            70.0,
            20.,
            BLACK,
        );
        draw_text(
            "Left, Right - change step",
            w as f32 + 10.0,
            110.0,
            20.,
            BLACK,
        );
        draw_text(
            "Up, Down - change draw size",
            w as f32 + 10.0,
            130.0,
            20.,
            BLACK,
        );
        draw_text(
            "B - enable showing zero step",
            w as f32 + 10.0,
            150.0,
            20.,
            BLACK,
        );

        next_frame().await
    }
}