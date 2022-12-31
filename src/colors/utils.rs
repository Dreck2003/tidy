use super::{BgColor, Colors, IColors, BOLD, ITALIC, RESET, UNDERLINE};

pub struct ColorBase<'a> {
    text: String,
    text_color: &'a str,
    with_bg: Option<&'a str>,
    is_bold: bool,
    is_italic: bool,
    is_underlined: bool,
}

impl<'a> ColorBase<'a> {
    fn new(color: Colors, text: &str) -> Self {
        Self {
            text: text.to_string(),
            text_color: color.get_color(),
            with_bg: None,
            is_bold: false,
            is_italic: false,
            is_underlined: false,
        }
    }
    pub fn bold(mut self) -> Self {
        self.is_bold = true;
        return self;
    }
    pub fn italic(mut self) -> Self {
        self.is_italic = true;
        return self;
    }
    pub fn bg(mut self, type_bg: BgColor) -> Self {
        self.with_bg = Some(type_bg.get_color());
        self
    }
    pub fn underline(mut self) -> Self {
        self.is_underlined = true;
        self
    }

    pub fn print(&self) {
        print!("{}", self.build())
    }
    pub fn println(&self) {
        println!("{}", self.build())
    }

    pub fn build(&self) -> String {
        let mut text = String::new();

        if self.is_bold {
            text.push_str(BOLD);
        }
        if self.is_italic {
            text.push_str(ITALIC);
        }
        if self.is_underlined {
            text.push_str(UNDERLINE)
        }

        if let Some(bg) = self.with_bg {
            text.push_str(bg);
        }

        text.push_str(self.text_color);
        text.push_str(&self.text);
        text.push_str(RESET);
        return text;
    }
}

impl<'a> std::ops::Add for ColorBase<'a> {
    type Output = String;

    fn add(self, rhs: Self) -> Self::Output {
        format!("{}{}", self.build(), rhs.build())
    }
}

pub struct Color;

impl Color {
    pub fn red(text: &str) -> ColorBase {
        return ColorBase::new(Colors::Red, text);
    }
    pub fn blue(text: &str) -> ColorBase {
        return ColorBase::new(Colors::Blue, text);
    }
    pub fn yellow(text: &str) -> ColorBase {
        return ColorBase::new(Colors::Yellow, text);
    }
    pub fn green(text: &str) -> ColorBase {
        return ColorBase::new(Colors::Green, text);
    }
    pub fn cyan(text: &str) -> ColorBase {
        return ColorBase::new(Colors::Cyan, text);
    }
    pub fn magenta(text: &str) -> ColorBase {
        return ColorBase::new(Colors::Magenta, text);
    }
    pub fn white(text: &str) -> ColorBase {
        return ColorBase::new(Colors::White, text);
    }
    pub fn black(text: &str) -> ColorBase {
        return ColorBase::new(Colors::Black, text);
    }
}
