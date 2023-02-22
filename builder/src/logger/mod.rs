pub mod colors;

#[derive(Clone)]
pub struct Logger {
    pub name: &'static str,
}

impl Logger {
    pub const fn new(name: &'static str) -> Self {
        return Self { name };
    }

    pub fn info(&self, message: String) {
        let kind_prefix = format!(
            "{}[{}INFO{}]{}",
            colors::FG_BLUE,
            colors::FG_CYAN,
            colors::FG_BLUE,
            colors::RESET
        );

        self.log(kind_prefix, message);
    }

    pub fn warn(&self, message: String) {
        let kind_prefix = format!(
            "{}[{}WARN{}]{}",
            colors::FG_YELLOW,
            colors::BRIGHT_FG_YELLOW,
            colors::FG_YELLOW,
            colors::RESET
        );

        self.log(kind_prefix, message);
    }

    pub fn debug(&self, message: String) {
        let kind_prefix = format!(
            "{}[{}DEBUG{}]{}",
            colors::FG_GREEN,
            colors::BRIGHT_FG_GREEN,
            colors::FG_GREEN,
            colors::RESET
        );

        self.log(kind_prefix, message);
    }

    pub fn error(&self, message: String) {
        let kind_prefix = format!(
            "{}[{}ERROR{}]{}",
            colors::FG_RED,
            colors::BRIGHT_FG_RED,
            colors::FG_RED,
            colors::RESET
        );

        self.log(kind_prefix, message);
    }

    pub fn log(&self, kind_prefix: String, message: String) {
        let name_prefix = format!(
            "{}[{}{}{}]{}",
            colors::FG_GREEN,
            colors::BRIGHT_FG_GREEN,
            self.name,
            colors::FG_GREEN,
            colors::RESET
        );

        let message_text = format!(
            "{} {} {}{}{}",
            kind_prefix,
            name_prefix,
            colors::RESET,
            message,
            colors::RESET
        );

        println!("{}", message_text);
    }
}
