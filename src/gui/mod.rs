use std::time::Instant;

use piston_window::{
    AdvancedWindow, Button, EventLoop, Key, MouseButton, MouseCursorEvent, PistonWindow,
    PressEvent, WindowSettings,
};

use crate::game::{Life, N_GRID};

pub fn show(life: &mut Life) {
    let rectangle_a = 10.0;
    let mut window: PistonWindow = WindowSettings::new("Game of life!", [640, 480])
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(5);
    window.set_capture_cursor(false);

    let mut running = false;

    let mut cursor = [0.0, 0.0];

    while let Some(event) = window.next() {
        event.mouse_cursor(|pos| {
            cursor = pos;
        });

        if let Some(btn) = event.press_args() {
            match btn {
                Button::Keyboard(key) => match key {
                    Key::Space => running = !running,
                    _ => {}
                },
                Button::Mouse(click) => {
                    if click == MouseButton::Left {
                        let x = (cursor[0] / rectangle_a) as usize;
                        let y = (cursor[1] / rectangle_a) as usize;
                        life.toggle(x, y);
                    }
                }
                _ => {}
            }
        }
        window.draw_2d(&event, |context, graphics, _device| {
            use piston_window::clear;
            if running {
                let start = Instant::now();
                life.next_generation();
                println!("Finished generation in: {:?}", start.elapsed());
            }
            clear([1.0; 4], graphics);
            let grid = life.current_generation();
            for x in 0..N_GRID {
                for y in 0..N_GRID {
                    if grid[x][y] {
                        piston_window::rectangle(
                            [0.0, 0.0, 0.0, 1.0],
                            [
                                x as f64 * rectangle_a,
                                y as f64 * rectangle_a,
                                rectangle_a,
                                rectangle_a,
                            ],
                            context.transform,
                            graphics,
                        );
                    }
                }
            }
        });
    }
}
