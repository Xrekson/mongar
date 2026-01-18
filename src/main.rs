use macroquad::prelude::*;

struct MainCircle {
    radius: f32,
    color: Color,
    position_x: f32,
    position_y: f32,
}

#[macroquad::main("game-x")]
async fn main() {
    let mut mainc = MainCircle {
        radius: 16.0,
        color: BLACK,
        position_x: screen_width() / 2.0,
        position_y: screen_height() / 2.0,
    };
    let ms: f32 = 200.0;
    loop {
        clear_background(GOLD);
        let delta_time = get_frame_time();
        move_element(&mut mainc, &ms, &delta_time);
        draw_circle(mainc.position_x, mainc.position_y, 16.0, mainc.color);
        next_frame().await
    }
}

fn move_element(main: &mut MainCircle, ms: &f32, delta: &f32) {
    if is_key_down(KeyCode::Right) {
        main.position_x += ms * delta;
    }
    if is_key_down(KeyCode::Left) {
        main.position_x -= ms * delta;
    }
    if is_key_down(KeyCode::Down) {
        main.position_y += ms * delta;
    }
    if is_key_down(KeyCode::Up) {
        main.position_y -= ms * delta;
    }

    main.position_x = clamp(
        main.position_x,
        0.0 + main.radius,
        screen_width() - main.radius,
    );
    main.position_y = clamp(
        main.position_y,
        0.0 + main.radius,
        screen_height() - main.radius,
    );
}
