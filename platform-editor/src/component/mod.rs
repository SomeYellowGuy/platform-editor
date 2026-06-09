use platform_editor_core::{
    component::{
        BackButtonBase, BackButtonMode, Hold,
        level::board::BoardBase,
        level_select::{LevelSelectHeaderBase, button::LevelSelectButtonBase},
        title::{TitleBase, button::ButtonBase},
    },
    screen::{Screen, TransitionCall, TransitionData},
};
use sdl3::{
    mouse::MouseButton,
    render::{BlendMode, FPoint},
};

use crate::{
    logic::{Logic, LogicData},
    render::{DrawResult, Render, RenderData},
    util,
};

pub mod level;
pub mod level_select;
pub mod title;

const BACK_BUTTON_INITIAL_RADIUS: f32 = 30.0;

impl Render for BackButtonBase {
    fn render(&self, data: &mut RenderData) -> DrawResult {
        let scale_multiplier = 1.0 + (self.hold_time as f32 / 3_000_000_000.0);
        let size = BACK_BUTTON_INITIAL_RADIUS * 2.0 * scale_multiplier;

        data.textures.back_button.set_blend_mode(BlendMode::Blend);
        data.textures
            .back_button
            .set_alpha_mod((255.0 * (1.0 - data.transition_offset(1.0 / 10.0).max(0.0))) as u8);

        data.canvas.copy_ex(
            &data.textures.back_button,
            None,
            util::frect_from_center(
                FPoint::new(self.pos.0 as f32, self.pos.1 as f32),
                size,
                size,
            ),
            0.0,
            None,
            false,
            false,
        )
    }
}

impl Logic for BackButtonBase {
    fn run_logic(&mut self, data: &mut LogicData) {
        let pos = FPoint::new(self.pos.0 as f32, self.pos.1 as f32);
        let mouse_pos = data.mouse_fpos();

        let (dx, dy) = (pos.x - mouse_pos.x, pos.y - mouse_pos.y);
        let distance_sq = dx * dx + dy * dy;

        let hovered = distance_sq <= BACK_BUTTON_INITIAL_RADIUS * BACK_BUTTON_INITIAL_RADIUS;

        self.update_hold_time(data.delta_time, hovered);

        if hovered && data.is_mouse_button_up(MouseButton::Left) {
            match self.mode {
                BackButtonMode::BackToTitle => {
                    data.set_transition_call(TransitionCall::Start(TransitionData::new(
                        800_000_000,
                        700_000_000,
                        Screen::Title,
                    )));
                }
            }
        }
    }
}

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
    BackButton(BackButtonBase),

    Title(TitleBase),
    Button(ButtonBase),

    LevelSelectButton(LevelSelectButtonBase),
    LevelSelectHeader(LevelSelectHeaderBase),

    Board(BoardBase),
}

impl_components! {
    BackButton,

    Title,
    Button,

    LevelSelectButton,
    LevelSelectHeader,

    Board
}
