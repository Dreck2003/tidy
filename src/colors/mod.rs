pub mod utils;

//Colors:
const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

//Background:
const BG_BLACK: &str = "\x1b[40m";
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";
const BG_WHITE: &str = "\x1b[47m";

//Reset:
const RESET: &str = "\x1b[0m";

// Decorations:
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";
const UNDERLINE: &str = "\x1b[4m";
const REVERSED: &str = "\x1b[7m";

trait IColors {
    fn get_color<'a>(&self) -> &'a str;
}

pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl IColors for Colors {
    fn get_color<'a>(&self) -> &'a str {
        match &self {
            Colors::Black => BLACK,
            Colors::Red => RED,
            Colors::Green => GREEN,
            Colors::Yellow => YELLOW,
            Colors::Blue => BLUE,
            Colors::Magenta => MAGENTA,
            Colors::Cyan => CYAN,
            Colors::White => WHITE,
        }
    }
}

pub enum BgColor {
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
}

impl IColors for BgColor {
    fn get_color<'a>(&self) -> &'a str {
        match self {
            BgColor::BgBlack => BG_BLACK,
            BgColor::BgRed => BG_RED,
            BgColor::BgGreen => BG_GREEN,
            BgColor::BgYellow => BG_YELLOW,
            BgColor::BgBlue => BG_BLUE,
            BgColor::BgMagenta => BG_MAGENTA,
            BgColor::BgCyan => BG_CYAN,
            BgColor::BgWhite => BG_WHITE,
        }
    }
}
