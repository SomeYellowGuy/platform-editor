use platform_editor_core::{
    component::{
        ComponentId,
        button::{ButtonBase, ButtonType},
        title::TitleBase,
    },
    screen::Screen,
};

use crate::{ComponentMap, NO_LOGIC_PRIORITY, component::Component};

pub fn on_enter(screen: Screen, map: &mut ComponentMap) {
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
            println!("9");
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
        Screen::LevelSelect => {}
        Screen::Level => {}
        Screen::Options => {}
    }
}
