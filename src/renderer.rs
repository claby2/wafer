use quick_error::quick_error;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::{FontError, InitError, Sdl2TtfContext};
use sdl2::video::{Window, WindowBuildError, WindowContext};
use sdl2::{EventPump, IntegerOrSdlError};

use crate::font::{WaferFont, WaferFontError, Writer};
use crate::term::grid::{Grid, GridDimensions};

// Graphic rendering abstraction layer

quick_error! {
    #[derive(Debug)]
    pub enum RenderError {
        TtfInitError(err : InitError) {
            from()
        }
        IntegerOrSdl(err : IntegerOrSdlError) {
            from()
        }
        WindowBuild(err : WindowBuildError) {
            from()
        }
        Font(err : FontError) {
            from()
        }
        Other(err : String) {
            from()
        }
        WaferFontError(err : WaferFontError) {
            from()
        }
    }
}

pub struct Renderer<'ttf, 's, 'a> {
    pub quit: bool,
    event_pump: EventPump,
    canvas: Canvas<Window>,
    writer: Writer<'ttf, 's, 'a>,
    texture_creator: TextureCreator<WindowContext>,
    grid: Grid,
}

impl<'ttf, 's, 'a> Renderer<'ttf, 's, 'a> {
    const CELL_WIDTH: u16 = 10;
    const CELL_HEIGHT: u16 = 20;

    pub fn new(ttf_context: &'ttf Sdl2TtfContext) -> Result<Renderer<'ttf, 's, 'a>, RenderError> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let event_pump = sdl_context.event_pump()?;

        let wafer_font = WaferFont::new("DejaVu Sans")?;
        let font = ttf_context.load_font(wafer_font.path, 128)?;
        let writer = Writer::new(font);

        const DEFAULT_WINDOW_WIDTH: u32 = 640;
        const DEFAULT_WINDOW_HEIGHT: u32 = 480;

        let window = video_subsystem
            .window("Wafer", DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
            .position_centered()
            .resizable()
            .build()?;

        let canvas = window.into_canvas().accelerated().build()?;
        let canvas_output_size = canvas.output_size()?;
        let texture_creator = canvas.texture_creator();

        Ok(Renderer {
            quit: false,
            event_pump,
            canvas,
            writer,
            texture_creator,
            grid: Grid::new(&GridDimensions::from_window_size(
                canvas_output_size.0,
                canvas_output_size.1,
            )),
        })
    }

    pub fn update(&mut self) -> Result<(), RenderError> {
        'running: for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.quit = true;
                    break 'running;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(_, _),
                    ..
                } => {
                    self.grid = Grid::new(&GridDimensions::from_window_size(
                        self.canvas.output_size()?.0,
                        self.canvas.output_size()?.1,
                    ));
                }
                _ => {}
            };
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), RenderError> {
        macro_rules! set_draw_color {
            ($color:expr) => {{
                self.canvas
                    .set_draw_color(Color::RGB($color.r, $color.g, $color.b));
            }};
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        let mut grid_string = String::from("");
        for (line, row) in self.grid.content.iter().enumerate() {
            let y = line as i32 * Self::CELL_HEIGHT as i32;
            for (column, cell) in row.iter().enumerate() {
                // Add cell character to grid string
                grid_string.push(cell.character);

                // Draw background
                let x = column as i32 * Self::CELL_WIDTH as i32;
                set_draw_color!(cell.background);
                let cell_rect = Rect::new(
                    x,
                    y,
                    u32::from(Self::CELL_WIDTH),
                    u32::from(Self::CELL_HEIGHT),
                );
                self.canvas.fill_rect(cell_rect)?;

                // Handle foreground content
                if cell.character != '\0' {
                    set_draw_color!(cell.foreground);
                    self.writer.set_surface(cell.character, &cell.foreground)?;
                    self.canvas.copy(
                        &self.writer.get_texture(&mut self.texture_creator)?,
                        None,
                        Some(cell_rect),
                    )?;
                }
            }
            grid_string.push('\n');
        }
        self.canvas.present();
        Ok(())
    }
}
