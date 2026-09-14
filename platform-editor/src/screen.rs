use platform_editor_core::{
    common_util::{self, ScrollInfo},
    component::{
        BackButtonBase, BackButtonMode, ComponentId,
        level::{board::BoardBase, item_tab::ItemTabBase},
        level_select::{LevelSelectHeaderBase, button::LevelSelectButtonBase},
        title::{
            TitleBase,
            button::{ButtonBase, ButtonType},
        },
    },
    level::scratch::levels::LEVEL_COUNT,
    screen::Screen,
};
use sdl3::mouse::MouseButton;

use crate::{
    ComponentMap, NO_LOGIC_PRIORITY, WIDTH,
    component::{
        Component,
        level_select::button::{LEVELS_PER_ROW, SPACING},
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

            for level in 0..LEVEL_COUNT {
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
        Screen::Level => {
            let mut base = BoardBase::new();
            let level = logic_data.map_or(0, |l| l.app_data.level.playing_level);
            base.state.load_scratch_level(level);
            map.insert(ComponentId::Board, Component::Board(base), 10, 0);
            map.insert(
                ComponentId::ItemTab,
                Component::ItemTab(ItemTabBase {}),
                15,
                5,
            );
        }
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
        Screen::Level => map.remove_all(|k| matches!(k, ComponentId::Board | ComponentId::ItemTab)),
        Screen::Options => {}
    }

    map.update_cache();
}

pub fn tick(screen: Screen, logic_data: &mut LogicData) {
    if screen == Screen::LevelSelect {
        let delta_seconds = logic_data.delta_time as f32 / 1_000_000_000.0;
        let pos = logic_data.mouse_fpos().y;

        common_util::scroll(ScrollInfo {
            held: logic_data.is_mouse_button_held(MouseButton::Left),
            last_pos: &mut logic_data.app_data.extra.last_y_mouse_pos,
            pos,
            velocity: &mut logic_data.app_data.level_select_scroll_velocity,
            scroll: &mut logic_data.app_data.level_select_scroll,
            delta_seconds,
            starting_level_select_scroll: STARTING_LEVEL_SELECT_SCROLL,
            spacing: SPACING,
            levels: LEVEL_COUNT,
            levels_per_row: LEVELS_PER_ROW,
            extra_end_scroll: 50.0,
        });
    }
}
