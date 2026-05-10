/// Options in the game.
#[derive(Debug)]
pub struct Options {
    pub sound: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { sound: true }
    }
}
