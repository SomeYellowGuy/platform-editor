use platform_editor_core::{
    component::{
        BackButtonBase, BackButtonMode, ComponentId,
        level_select::{LevelSelectHeaderBase, button::LevelSelectButtonBase},
        title::{
            TitleBase,
            button::{ButtonBase, ButtonType},
        },
    },
    screen::Screen,
};
use sdl3::mouse::MouseButton;

use crate::{
    ComponentMap, LEVELS, NO_LOGIC_PRIORITY, WIDTH,
    component::{
        Component,
        level_select::{self, button::LEVELS_PER_ROW},
    },
    logic::LogicData,
};

/// The lnitial scroll value for the level select.
pub const STARTING_LEVEL_SELECT_SCROLL: f32 = 60.0;

/// Called when a screen is entered into.
///
/// `logic_data` is `Some` if this is because of a transition.
pub fn on_enter(screen: Screen, map: &mut ComponentMap, logic_data: Option<&mut LogicData>) {
    match screen {
        Screen::Title => {
            // Add the title and buttons.
            map.insert(
                ComponentId::Title,
                Component::Title(TitleBase),
                0,
                NO_LOGIC_PRIORITY,
            );

            for ty in ButtonType::ALL {
                map.insert(
                    ComponentId::Button(ty),
                    Component::Button(ButtonBase::new(ty)),
                    0,
                    0,
                );
            }
        }
        Screen::LevelSelect => {
            if let Some(logic_data) = logic_data {
                logic_data.app_data.level_select_scroll_velocity = 0.0;
            }

            for level in 0..LEVELS {
                map.insert(
                    ComponentId::LevelSelectButton(level),
                    Component::LevelSelectButton(LevelSelectButtonBase::new(level)),
                    0,
                    0,
                );
            }

            map.insert(
                ComponentId::LevelSelectHeader,
                Component::LevelSelectHeader(LevelSelectHeaderBase),
                10,
                NO_LOGIC_PRIORITY,
            );

            map.insert(
                ComponentId::BackButton,
                Component::BackButton(BackButtonBase::new(
                    (WIDTH as i32 - 50, 45),
                    BackButtonMode::BackToTitle,
                )),
                15,
                0,
            );
        }
        Screen::Level => {}
        Screen::Options => {}
    }

    map.update_cache();
}

pub fn on_exit(screen: Screen, map: &mut ComponentMap) {
    match screen {
        Screen::Title => {
            map.remove_all(|k| matches!(k, ComponentId::Title | ComponentId::Button(_)))
        }
        Screen::LevelSelect => map.remove_all(|k| {
            matches!(
                k,
                ComponentId::LevelSelectButton(_)
                    | ComponentId::LevelSelectHeader
                    | ComponentId::BackButton
            )
        }),
        Screen::Level => {}
        Screen::Options => {}
    }

    map.update_cache();
}

pub fn tick(screen: Screen, logic_data: &mut LogicData) {
    match screen {
        Screen::LevelSelect => {
            let delta_seconds = logic_data.delta_time as f32 / 1_000_000_000.0;

            if logic_data.is_mouse_button_held(MouseButton::Left) {
                let pos = logic_data.mouse_fpos().y;
                if let Some(prev_pos) = logic_data.app_data.extra.last_y_mouse_pos {
                    let delta = pos - prev_pos;
                    logic_data.app_data.level_select_scroll_velocity = delta;
                }
                logic_data.app_data.extra.last_y_mouse_pos = Some(pos);
            } else {
                let max_scroll = STARTING_LEVEL_SELECT_SCROLL
                    - level_select::button::SPACING
                        * (LEVELS.div_ceil(LEVELS_PER_ROW) - 2).max(0) as f32
                    - 50.0;
                // Push the scroll towards the level buttons if it is dragged out of bounds.
                let (drag_value, out_by): (f32, f32) =
                    if logic_data.app_data.level_select_scroll > STARTING_LEVEL_SELECT_SCROLL {
                        logic_data.app_data.level_select_scroll_velocity -= delta_seconds * 100.0;
                        (
                            0.01,
                            STARTING_LEVEL_SELECT_SCROLL - logic_data.app_data.level_select_scroll,
                        )
                    } else if logic_data.app_data.level_select_scroll < max_scroll {
                        logic_data.app_data.level_select_scroll_velocity += delta_seconds * 100.0;
                        (0.01, max_scroll - logic_data.app_data.level_select_scroll)
                    } else {
                        (0.01, 0.0)
                    };
                logic_data.app_data.level_select_scroll_velocity *= drag_value.powf(delta_seconds);
                // Max out the velocity if needed.
                logic_data.app_data.level_select_scroll_velocity = logic_data
                    .app_data
                    .level_select_scroll_velocity
                    .clamp(-10.0 + out_by / 10.0, 10.0 + out_by / 10.0);
                logic_data.app_data.extra.last_y_mouse_pos = None;
            }
            logic_data.app_data.level_select_scroll +=
                logic_data.app_data.level_select_scroll_velocity * delta_seconds * 40.0;
        }
        _ => {}
    }
}
