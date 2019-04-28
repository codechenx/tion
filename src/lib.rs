use termion;
use termion::{color, cursor, clear};
use std::fmt;

const BoxVertBar: char = '\u{2500}';
const BoxHorBar: char = '\u{2502}';
const BoxTopLeftCorner: char = '\u{250c}';
const BoxTopRightCorner: char = '\u{2510}';
const BoxBottomRightCorner: char = '\u{2518}';
const BoxBottomLeftCorner: char = '\u{2514}';
const BoxEllipsis: char = '\u{2026}';

enum Color{
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    White,
    Yellow
}


trait Draw {
    fn new() -> Self;
    fn set_rect(&mut self, x: usize, y: usize, width: usize, height: usize);
    fn get_rect(&self) -> (usize, usize, usize, usize);
    fn set_foreground_color(&self);
    fn set_background_color(&self);
    fn draw(&self);
    //fn InputHandler(&self);
    //fn Focus(&self);
    //fn Blur(&self);
}

trait Focus{
    fn focusable();
    fn focus();
    fn set_focus_color();
    fn blur();
}



pub struct Box {
    pub title: String,
    pub size: (usize, usize, usize, usize),
    pub border: bool,
    pub autoSize: bool,
    pub foregroundColor: fmt::Result, 
    pub backgroundColor: fmt::Result
}

impl Box {
    fn setAutoSize(&mut self) {
        self.autoSize = true;
    }
    fn setBorder(&mut self) {
        self.border = true;
    }
}

impl Draw for Box {
    fn new() -> Self {
        Box {
            title: String::from("untitled"),
            size: termion::terminal_size(),
            border: false,
            autoSize: true,
            foregroundColor:  color::Fg,
            backgroundColor:  color::Fg(color::Red)
        }
    }

    fn set_rect(&mut self, x: usize, y: usize, width: usize, height: usize){
        self.size = (x, y, width, height)
    }

    fn get_rect(&self) -> (usize, usize, usize, usize){
        self.size
    }


    fn draw(&self) {}
    //fn SetRect(&self, x: usize, y: usize, width: usize, height: usize){}
    //fn GetRect(&self) -> (usize, usize, usize, usize){}
}
