use std::fmt::{Display, Formatter};

use bevy::{platform::collections::HashMap, prelude::*};
use smallvec::{SmallVec, smallvec};

use crate::ui::UiEvent;

pub struct ShortcutsPlugin;

impl Plugin for ShortcutsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Shortcuts::default())
            .add_systems(Update, handle_shortcuts);
    }
}

#[derive(Resource, Debug)]
pub struct Shortcuts {
    pub shortcuts: HashMap<UiEvent, Shortcut>
}

impl Shortcuts {
    pub fn get_shortcut(&self, event: UiEvent) -> Option<Shortcut> {
        self.shortcuts.get(&event).cloned()
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Shortcut {
    pub keys: SmallVec<[KeyCode; 3]>,
}

fn format_keycode(keycode: KeyCode) -> String {
    match keycode {
        KeyCode::ControlLeft => "Ctrl".to_string(),
        KeyCode::ShiftLeft => "Shift".to_string(),
        KeyCode::AltLeft => "Alt".to_string(),
        KeyCode::SuperLeft => {
            if cfg!(target_os = "windows") {
                "Win".to_string()
            } else {
                "Cmd".to_string()
            }
        },
        _ => {
            let formatted = format!("{:?}", keycode);
            // if starts with "Key", remove the "Key"
            if formatted.starts_with("Key") {
                formatted[3..].to_string()
            } else {
                formatted
            }
        }
    }
}

impl Display for Shortcut {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}", self.keys.iter().map(|key| format_keycode(*key)).collect::<Vec<String>>().join("+"))
    }
}


impl Default for Shortcuts {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(UiEvent::FileNew, Shortcut { keys: smallvec![KeyCode::ControlLeft, KeyCode::KeyN] });
        map.insert(UiEvent::FileOpen, Shortcut { keys: smallvec![KeyCode::ControlLeft, KeyCode::KeyO] });
        map.insert(UiEvent::FileSave, Shortcut { keys: smallvec![KeyCode::ControlLeft, KeyCode::KeyS] });
        map.insert(UiEvent::FileSaveAs, Shortcut { keys: smallvec![KeyCode::ControlLeft, KeyCode::ShiftLeft, KeyCode::KeyS] });
        map.insert(UiEvent::FileClose, Shortcut { keys: smallvec![KeyCode::ControlLeft, KeyCode::KeyW] });
        map.insert(UiEvent::FileExit, Shortcut { keys: smallvec![KeyCode::AltLeft, KeyCode::F4] });
        Self { shortcuts: map }
    }
}

pub fn handle_shortcuts(
    shortcuts: Res<Shortcuts>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_event_writer: MessageWriter<UiEvent>,
) {
    for (event, shortcut) in shortcuts.shortcuts.iter() {
        if keys.all_pressed(shortcut.keys.iter().cloned()) {
            if keys.any_just_pressed(shortcut.keys.iter().cloned()) {
                info!("Shortcut pressed {:?} --> {:?}", shortcut, event);
                ui_event_writer.write(event.clone());
            }
        }
    }
}