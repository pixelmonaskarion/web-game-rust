extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate touch_visualizer;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

use touch_visualizer::TouchVisualizer;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::HashMap;
use piston::window::{ AdvancedWindow, Window, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
use opengl_graphics::*;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

type AxisValues = HashMap<(u32, u8), f64>;

mod game;
use game::Game;

mod connection;
use connection::Connection;

#[tokio::main]
async fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow = WindowSettings::new("piston-example-user_input", [1000, 1000])
        .exit_on_esc(true).graphics_api(opengl).build().unwrap();

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let ref mut gl = GlGraphics::new(opengl);

    let mut touch_visualizer = TouchVisualizer::new();
    let mut axis_values: AxisValues = HashMap::new();
    
    let font = "C:/WINDOWS/FONTS/ARIAL.ttf";
    let mut glyphs = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

    let mut game = Game::new();
    //prod
    //let connection = Connection::new("http://71.197.148.230".to_string());
    //debug
    let connection = Connection::new("https://minecraft.net/".to_string());

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        touch_visualizer.event(window.size(), &e);
        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == MouseButton::Left {
                game.mouse_buttons[0] = true;
            } else if button == MouseButton::Right {
                game.mouse_buttons[1] = true;
            } else if button == MouseButton::Middle {
                game.mouse_buttons[2] = true;
            } else {
                println!("Pressed mouse button '{:?}'", button);
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("Turned capture cursor on");
                capture_cursor = !capture_cursor;
                window.set_capture_cursor(capture_cursor);
            }
            while game.keys.len() <= key.code() as usize {
                game.keys.push(false);
            }
            game.keys[key.code() as usize] = true;
            //println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => {
                    if game.keys.len() > key.code() as usize {
                        game.keys[key.code() as usize] = false;
                    }
                    //println!("Released keyboard key '{:?}'", key)
                },
                Button::Mouse(button) => {
                    if button == MouseButton::Left {
                        game.mouse_buttons[0] = false;
                    } else if button == MouseButton::Right {
                        game.mouse_buttons[1] = false;
                    } else if button == MouseButton::Middle {
                        game.mouse_buttons[2] = false;
                    } else {
                        println!("Released mouse button '{:?}'", button);
                    }
                    
                },
                Button::Controller(button) => println!("Released controller button '{:?}'", button),
                Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
            }
        };
        if let Some(args) = e.controller_axis_args() {
            axis_values.insert((args.id, args.axis), args.position);
        }
        e.mouse_cursor(|pos| {
            game.mouse_pos = pos;
            futures::executor::block_on(connection.send_mouse(pos));
            //println!("Mouse moved '{} {}'", pos[0], pos[1]);
        });
        /*e.mouse_scroll(|d| println!("Scrolled mouse '{}, {}'", d[0], d[1]));
        e.mouse_relative(|d| println!("Relative mouse moved '{} {}'", d[0], d[1]));
        e.text(|text| println!("Typed '{}'", text));*/
        e.resize(|args| println!("Resized '{}, {}'", args.window_size[0], args.window_size[1]));
        if let Some(cursor) = e.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse left"); }
        };
        if let Some(args) = e.render_args() {
            game.render(gl, &mut glyphs, args);
        }
        if let Some(_args) = e.idle_args() {
            // println!("Idle {}", _args.dt);
        }
        if let Some(args) = e.update_args() {
            game.update(args);
            futures::executor::block_on(get_mice(&connection, &mut game));
            /*
            // Used to test CPU overload.
            println!("Update {}", _args.dt);
            let mut x: f64 = 0.0;
            for _ in 0..500_000 {
                x += (1.0 + x).sqrt();
            }
            println!("{}", x);
            */
        }
    }
}

async fn get_mice(connection: &Connection, game: &mut Game) {
    let result = connection.get_mice().await;
    match result {
        Ok(e) => {
            println!("{}", e.mice.len());
            game.other_mice = e.mice
        },
        Err(e) => {println!("{}", e)}
    }
}