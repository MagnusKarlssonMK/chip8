//! # Display
//!
//! Contains the Display module, creating a canvas for an SLD2 context,
//! and draws pixels according to the input display buffer.
extern crate sdl2;

use sdl2::{Sdl, pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::error::Error;

const ON_COLOR: Color = Color::RGB(255, 255, 255);
const OFF_COLOR: Color = Color::RGB(0, 0, 0);

const SCALING: u32 = 20;

pub struct Display {
    canvas: Canvas<Window>,
    screen_width: u32,
    screen_height: u32,
}

impl Display {
    /// Creates a new Display instance.
    pub fn new(
        sdl_context: &Sdl,
        screen_width: u8,
        screen_height: u8,
    ) -> Result<Self, Box<dyn Error>> {
        let gfx = sdl_context.video()?;
        let screen_width = screen_width as u32;
        let screen_height = screen_height as u32;
        let window = gfx
            .window(
                "CHIP-8 display",
                screen_width * SCALING,
                screen_height * SCALING,
            )
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;
        Ok(Self {
            canvas,
            screen_width,
            screen_height,
        })
    }

    /// Updates the display with the input display buffer.
    /// The length of the display_data should match the total number of pixels.
    /// If it's too short, the remaining pixels will be left at background color.
    /// If it's too long, the overshooting elements will be ignored.
    pub fn draw_screen(&mut self, display_data: &[bool]) {
        self.canvas.set_draw_color(OFF_COLOR);
        self.canvas.clear();

        self.canvas.set_draw_color(ON_COLOR);

        for (i, v) in display_data.iter().enumerate() {
            if i > self.screen_width as usize * self.screen_height as usize {
                break;
            }
            if *v {
                let x = (i as u32 % self.screen_width) * SCALING;
                let y = (i as u32 / self.screen_width) * SCALING;
                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALING, SCALING));
            }
        }

        self.canvas.present();
    }
}
