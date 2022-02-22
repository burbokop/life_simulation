use std::time::Duration;

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect::Rect};

mod cell;
mod scene;
mod brains;

use crate::{scene::Scene, cell::Cell, brains::DefaultBrain};

fn main() {


    let mut scene = Scene::default();
    let brain = DefaultBrain::default();


    let mut rng = rand::rngs::OsRng::default();


    scene.cells.push(Cell::new_random(8, 0..20, &mut rng, 1, 1));
    scene.cells.push(Cell::new_random(8, 0..20, &mut rng, 2, 2));
    scene.cells.push(Cell::new_random(8, 0..20, &mut rng, 1, 2));
    scene.cells.push(Cell::new_random(8, 0..20, &mut rng, 3, 2));


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i: i32 = 0;

    const CELL_W: u32 = 20;
    const CELL_H: u32 = 20;

    'running: loop {

        let color_factor = (((i as f64 * 0.01).sin() + 1.) * 127.) as u8;
        i += 1;


        canvas.set_draw_color(Color::RGB(color_factor, 64, 255 - color_factor));
        canvas.clear();

        scene.proceed(&brain);

        scene.cells.iter().for_each(|cell|{
            let rect = Rect::new(cell.x * CELL_W as i32, cell.y * CELL_H as i32, CELL_W, CELL_H);
            canvas.set_draw_color(Color::RGB(255 - color_factor, 64, color_factor));
            canvas.fill_rect(rect).unwrap();
            canvas.set_draw_color(Color::RGB(255 - color_factor, color_factor, 64));
            canvas.draw_rect(rect).unwrap();
        });


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
