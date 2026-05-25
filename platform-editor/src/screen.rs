use platform_editor_core::{
    component::{
        ComponentId,
        button::{ButtonBase, ButtonType},
        level_select_button::LevelSelectButtonBase,
        title::TitleBase,
    },
    screen::Screen,
};
use sdl3::mouse::MouseButton;

use crate::{ComponentMap, NO_LOGIC_PRIORITY, component::Component, logic::LogicData};

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

            for level in 0..30 {
                map.insert(
                    ComponentId::LevelSelectButton(level),
                    Component::LevelSelectButton(LevelSelectButtonBase::new(level)),
                    0,
                    0,
                );
            }
        }
        Screen::Level => {}
        Screen::Options => {}
    }
}

pub fn on_exit(screen: Screen, map: &mut ComponentMap) {
    match screen {
        Screen::Title => {
            map.remove_all(|k| matches!(k, ComponentId::Title | ComponentId::Button(_)))
        }
        Screen::LevelSelect => map.remove_all(|k| matches!(k, ComponentId::LevelSelectButton(_))),
        Screen::Level => {}
        Screen::Options => {}
    }
}

pub fn tick(screen: Screen, logic_data: &mut LogicData) {
    match screen {
        Screen::LevelSelect => {
            if logic_data.is_mouse_button_held(MouseButton::Left) {
                let pos = logic_data.mouse_fpos().y;
                if let Some(prev_pos) = logic_data.app_data.extra.last_y_mouse_pos {
                    let delta = pos - prev_pos;
                    logic_data.app_data.level_select_scroll_velocity = delta;
                    // Max out the velocity if needed.
                    logic_data.app_data.level_select_scroll_velocity = logic_data
                        .app_data
                        .level_select_scroll_velocity
                        .clamp(-20.0, 20.0);
                }
                logic_data.app_data.extra.last_y_mouse_pos = Some(pos);
            } else {
                logic_data.app_data.level_select_scroll_velocity *=
                    0.01_f32.powf(logic_data.delta_time as f32 / 1_000_000_000.0);
                logic_data.app_data.extra.last_y_mouse_pos = None;
            }
            logic_data.app_data.level_select_scroll +=
                logic_data.app_data.level_select_scroll_velocity;
        }
        _ => {}
    }
}
