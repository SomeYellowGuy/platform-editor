use crate::{component::title::TitleBase, logic::Logic, render::Render};

pub mod title;

/// Represents something that can hold logic and be rendered.
pub enum Component {
    Title,
}

impl Render for Component {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        match self {
            Self::Title => TitleBase.render(data),
        }
    }
}

impl Logic for Component {
    fn run_logic(&mut self, _app_data: &mut crate::AppData) {
        match self {
            _ => {}
        }
    }
}