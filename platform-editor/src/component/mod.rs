use crate::{component::title::TitleBase, render::Render};

pub mod title;

/// Represents something that can hold logic and be rendered.
pub enum Component {
    Title(TitleBase),
}

impl Render for Component {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        match self {
            Self::Title(title) => title.render(data),
        }
    }
}
