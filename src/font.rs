use std::path::PathBuf;

use font_kit::error::FontLoadingError;
use font_kit::error::SelectionError;
use font_kit::handle::Handle;
use font_kit::source::SystemSource;

use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, TextureValueError};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, FontError};
use sdl2::video::WindowContext;

use quick_error::quick_error;

use crate::term::cell::WaferColor;

// Font sourcing and rendering interface

quick_error! {
    #[derive(Debug)]
    pub enum WaferFontError {
        Selection(err : SelectionError) {
            from()
        }
        FontLoading(err : FontLoadingError) {
            from()
        }
        TextureValue(err : TextureValueError) {
            from()
        }
        Other(err : String) {
            from()
        }
    }
}

#[derive(Debug)]
pub struct WaferFont {
    pub path: PathBuf,
}

impl WaferFont {
    pub fn new(name: &str) -> Result<Self, WaferFontError> {
        let handle = SystemSource::new().select_by_postscript_name(name)?;
        let path = match handle {
            Handle::Path { ref path, .. } => Ok(path),
            _ => Err(WaferFontError::Other(format!(
                "Could not get path to font: {}",
                name
            ))),
        }?;
        Ok(Self {
            path: path.to_path_buf(),
        })
    }
}

pub struct Writer<'ttf, 's, 'a> {
    pub font: Font<'ttf, 's>,
    pub surface: Option<Surface<'a>>,
}

impl<'ttf, 's, 'a> Writer<'ttf, 's, 'a> {
    pub fn new(font: Font<'ttf, 's>) -> Writer<'ttf, 's, 'a> {
        Writer {
            font,
            surface: None,
        }
    }

    pub fn set_surface(&mut self, character: char, color: &WaferColor) -> Result<(), FontError> {
        let mut byte_buffer = [0; 4];
        self.surface = Some(
            self.font
                .render(character.encode_utf8(&mut byte_buffer))
                .blended(Color::RGB(color.r, color.g, color.b))?,
        );
        Ok(())
    }

    pub fn get_texture<'b>(
        &'a self,
        texture_creator: &'a mut TextureCreator<WindowContext>,
    ) -> Result<Texture, WaferFontError> {
        match &self.surface {
            Some(surface) => Ok(texture_creator.create_texture_from_surface(&surface)?),
            None => Err(WaferFontError::Other(
                "Font surface not initialized".to_string(),
            )),
        }
    }
}
