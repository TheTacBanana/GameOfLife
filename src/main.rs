use std::collections::btree_map::Range;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use rand::Rng;
 
const GAME_SIZE : (i32, i32) = (2560, 1600);
const PIXEL_SIZE : u32 = 1;

const SURVIVE : [bool; 10] = [false, false, false, true,  true, false, false, false, false, false];
const BIRTH   : [bool; 10] = [false, false, false, true, false, false, false, false, false, false];

fn step(board : &[bool; (GAME_SIZE.0 * GAME_SIZE.1) as usize], new_board : &mut[bool; (GAME_SIZE.0 * GAME_SIZE.1) as usize]){
    let mut sum : usize;
    let mut pos : usize;
    for x in 0..GAME_SIZE.0 {
        for y in 0..GAME_SIZE.1 {
            pos = (y * GAME_SIZE.0 + x) as usize;
            sum = 0;
            for yc in -1 as i32..=1 {
                let y_index = (((y + yc) % GAME_SIZE.1) + GAME_SIZE.1) % GAME_SIZE.1;
                for xc in -1 as i32..=1 {
                    let x_index = (((x + xc) % GAME_SIZE.0) + GAME_SIZE.0) % GAME_SIZE.0;
                    sum += board[(y_index * GAME_SIZE.0 + x_index) as usize] as usize;
                }
            }
            new_board[pos] = match board[pos] {
                true => { SURVIVE[sum] },
                false => { BIRTH[sum] }, 
            }
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let mut window = video_subsystem.window("Game of Life", GAME_SIZE.0 as u32 * PIXEL_SIZE, GAME_SIZE.1 as u32 * PIXEL_SIZE)
        .position_centered()
        .build()
        .unwrap();
    //window.set_fullscreen(sdl2::video::FullscreenType::True).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut buffer_1 = [false; (GAME_SIZE.0 * GAME_SIZE.1) as usize];
    let mut buffer_2 = [false; (GAME_SIZE.0 * GAME_SIZE.1) as usize];
    let mut front_reference = &mut buffer_1;
    let mut back_reference = &mut buffer_2;

    let mut pause : bool = true;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    pause = !pause;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let mut rng = rand::thread_rng();
                    for _ in 0..((GAME_SIZE.0 * GAME_SIZE.1) / 2){
                        back_reference[rng.gen_range(0..(GAME_SIZE.0 * GAME_SIZE.1)) as usize] = true;
                    }
                },
                _ => {}
            }
        }

        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left){
            let state = event_pump.mouse_state();
            pause = true;
            back_reference[((state.y() / PIXEL_SIZE as i32 ) * GAME_SIZE.0 + (state.x() / PIXEL_SIZE as i32)) as usize] = true;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        if !pause{ 
            step(&back_reference, &mut front_reference); 
            let temp = front_reference;
            (front_reference, back_reference) = (back_reference, temp);
        }

        for x in 0..GAME_SIZE.0 {
            for y in 0..GAME_SIZE.1 {
                if back_reference[(y * GAME_SIZE.0 + x) as usize]{
                    canvas.fill_rect(Rect::new(x * PIXEL_SIZE as i32, y * PIXEL_SIZE as i32, PIXEL_SIZE, PIXEL_SIZE)).unwrap();
                }
            }
        }

        canvas.present();
    }
}
