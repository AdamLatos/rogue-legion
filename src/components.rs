use rltk::RGB;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Debug, PartialEq)]
pub struct Player {}

#[derive(Debug, PartialEq)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
}
