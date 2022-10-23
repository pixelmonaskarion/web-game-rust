use opengl_graphics::*;
use piston::input::*;
use std::collections::HashMap;

pub struct Game {
    pub mouse_buttons: [bool; 3],
    pub keys: Vec<bool>,
    pub mouse_pos: [f64; 2],
    pub other_mice: HashMap<String, [f64; 2]>
}

impl Game {
    pub fn new() -> Self {
        Self {
            mouse_buttons: [false; 3],
            keys: Vec::new(),
            mouse_pos: [0.0,0.0],
            other_mice: HashMap::new(),
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, glyphs: &mut GlyphCache, args: RenderArgs) {
        gl.draw(args.viewport(), |c, g| {
            graphics::clear([1.0; 4], g);
            graphics::ellipse(
                [0.0, 1.0, 0.0, 1.0],
                graphics::ellipse::circle(self.mouse_pos[0], self.mouse_pos[1], 5.0),
                c.transform, g);

            for (name, mouse) in &self.other_mice {
                graphics::ellipse(
                    [0.0, 1.0, 0.0, 1.0],
                    graphics::ellipse::circle(mouse[0], mouse[1], 5.0),
                    c.transform, g);
            }
        }
    );
    }

    pub fn update(&mut self, _args: UpdateArgs) {

    }
}