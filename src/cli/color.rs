pub trait Color {
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn cyan(self) -> String; // Added cyan for more styling options
}

impl<T> Color for T
where
    T: ToString,
{
    fn red(self) -> String {
        format!("\x1b[31m{}\x1b[0m", self.to_string())
    }
    fn green(self) -> String {
        format!("\x1b[32m{}\x1b[0m", self.to_string())
    }
    fn yellow(self) -> String {
        format!("\x1b[33m{}\x1b[0m", self.to_string())
    }
    fn blue(self) -> String {
        format!("\x1b[34m{}\x1b[0m", self.to_string())
    }
    fn cyan(self) -> String {
        format!("\x1b[36m{}\x1b[0m", self.to_string())
    }
}
