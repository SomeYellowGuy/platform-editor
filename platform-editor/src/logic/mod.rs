use crate::AppData;

/// A trait to provide a method for a "logical tick", which may or may not
/// affect the component itself.
pub trait Logic {
    /// Performs this object's logic.
    fn run_logic(&mut self, app_data: &mut AppData);
}