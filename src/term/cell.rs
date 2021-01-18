#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub character: char,
    pub foreground: WaferColor,
    pub background: WaferColor,
}

impl Cell {
    pub const CELL_WIDTH: u16 = 10;
    pub const CELL_HEIGHT: u16 = 20;

    pub fn new() -> Self {
        Self {
            character: '\0',
            // TODO: Introduce theming system for cell color defaults
            foreground: WaferColor::new(255, 255, 255),
            background: WaferColor::new(0, 0, 0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct WaferColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl WaferColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
