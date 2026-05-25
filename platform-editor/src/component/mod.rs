use platform_editor_core::component::{
    title::button::ButtonBase, level_select::button::LevelSelectButtonBase, title::logo::TitleBase,
};

use crate::{
    logic::{Logic, LogicData},
    render::{DrawResult, Render, RenderData},
};

pub mod level_select;
pub mod title;

macro_rules! impl_components {
    ( $($variant:ident),+ ) => {
        impl Render for Component {
            fn render(&self, data: &mut RenderData) -> DrawResult {
                match self {
                    $( Self:: $variant(o) => o.render(data), )+
                }
            }
        }

        impl Logic for Component {
            fn run_logic(&mut self, data: &mut LogicData) {
                match self {
                    $( Self:: $variant(o) => o.run_logic(data), )+
                }
            }
        }
    };
}

/// Represents something that can hold logic and be rendered.
pub enum Component {
    Title(TitleBase),
    Button(ButtonBase),

    LevelSelectButton(LevelSelectButtonBase),
}

impl_components! {
    Title,
    Button,

    LevelSelectButton
}
