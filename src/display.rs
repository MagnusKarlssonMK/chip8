//! # Display
//!
//! Contains the Display module, creating a canvas for an SLD2 context,
//! and draws pixels according to the input display buffer.
extern crate sdl2;

use crate::chip8options;
use sdl2::{Sdl, pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::error::Error;

pub struct Display {
    canvas: Canvas<Window>,
    screen_width: u32,
    screen_height: u32,
    scaling: u32,
    on_color: Color,
    off_color: Color,
}

impl Display {
    /// Creates a new Display instance.
    pub fn new(
        sdl_context: &Sdl,
        disp_options: &chip8options::Display,
    ) -> Result<Self, Box<dyn Error>> {
        let gfx = sdl_context.video()?;
        let screen_width = disp_options.display_width;
        let screen_height = disp_options.display_height;
        let scaling = disp_options.scaling;
        let window = gfx
            .window(
                "CHIP-8 emulator",
                screen_width * scaling,
                screen_height * scaling,
            )
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;
        Ok(Self {
            canvas,
            screen_width,
            screen_height,
            scaling,
            on_color: Color::RGB(
                disp_options.color_on_rgb.0,
                disp_options.color_on_rgb.1,
                disp_options.color_on_rgb.1,
            ),
            off_color: Color::RGB(
                disp_options.color_off_rgb.0,
                disp_options.color_off_rgb.1,
                disp_options.color_off_rgb.1,
            ),
        })
    }

    /// Updates the display with the input display buffer.
    /// The length of the display_data should match the total number of pixels.
    /// If it's too short, the remaining pixels will be left at background color.
    /// If it's too long, the overshooting elements will be ignored.
    pub fn draw_screen(&mut self, display_data: &[bool]) {
        self.canvas.set_draw_color(self.off_color);
        self.canvas.clear();

        self.canvas.set_draw_color(self.on_color);

        for (i, v) in display_data.iter().enumerate() {
            if i > self.screen_width as usize * self.screen_height as usize {
                break;
            }
            if *v {
                let x = (i as u32 % self.screen_width) * self.scaling;
                let y = (i as u32 / self.screen_width) * self.scaling;
                let _ = self.canvas.fill_rect(Rect::new(
                    x as i32,
                    y as i32,
                    self.scaling,
                    self.scaling,
                ));
            }
        }

        self.canvas.present();
    }
}
