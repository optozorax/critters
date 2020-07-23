mod world;

use macroquad::*;

use world::*;

#[macroquad::main("Life")]
async fn main() {
    let w = 500;
    let h = 500;

	let mut world = World::new(w/2, h/2, false);

    let mut buffer = vec![WHITE; w * h];
    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture = load_texture_from_image(&image);

    let mut size = 3;

    let mut i = 0i64;

    loop {
        clear_background(BLUE);

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

        let (mouse_x, mouse_y) = mouse_position();
        let mouse_x = mouse_x as usize;
        let mouse_y = mouse_y as usize;

        if is_mouse_button_down(MouseButton::Left) {
            for x in 0..size {
                for y in 0..size {
                    world.set(mouse_x + x, mouse_y + y, true);
                }
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            for x in 0..size {
                for y in 0..size {
                    world.set(mouse_x + x, mouse_y + y, false);
                }
            }
        }

        buffer.iter_mut().zip(world.arr.iter()).for_each(|(buffer, &world)| {
            *buffer = if world { WHITE } else { BLACK }
        });

        for x in 0..size {
            for y in 0..size {
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

        next_frame().await
    }
}