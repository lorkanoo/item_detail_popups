use crate::addon::Addon;
use device_query::{DeviceQuery, DeviceState};
use enigo::{
    Button, Direction,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::time::Duration;
use std::{fmt, thread};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Default)]
pub struct KeyCombination {
    pub key: Option<char>,
    pub key_code: Option<u32>,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub left_click: bool,
}

impl KeyCombination {
    pub fn shift_click() -> Self {
        KeyCombination {
            left_click: true,
            shift: true,
            ..KeyCombination::default()
        }
    }
    pub fn select_all() -> Self {
        KeyCombination {
            key: Some('a'),
            ctrl: true,
            ..KeyCombination::default()
        }
    }
    pub fn cut() -> Self {
        KeyCombination {
            key: Some('x'),
            ctrl: true,
            ..KeyCombination::default()
        }
    }
    pub fn enter() -> Self {
        KeyCombination {
            key_code: Some(13),
            ..KeyCombination::default()
        }
    }
    pub fn backspace() -> Self {
        KeyCombination {
            key_code: Some(8),
            ..KeyCombination::default()
        }
    }
}

impl fmt::Display for KeyCombination {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = "".to_string();
        if self.ctrl {
            result.push_str("Ctrl+");
        }
        if self.shift {
            result.push_str("Shift+");
        }
        if self.alt {
            result.push_str("Alt+");
        }
        if let Some(ch) = &self.key {
            result.push(*ch);
        }

        write!(f, "{}", result)
    }
}

fn all_keys_released() -> bool {
    let device_state = DeviceState::new();
    let keys = device_state.get_keys();
    debug!("[all_keys_released]: not released keys: {keys:?}");
    keys.is_empty()
}

pub fn trigger_key_combination(key_combination: &KeyCombination) {
    debug!("[trigger_key_combination] combination: {key_combination:?}");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut retries = 75;
    let should_wait_for_key_release = Addon::read_config().wait_until_all_keys_released;
    while should_wait_for_key_release && retries > 0 {
        if all_keys_released() {
            break;
        }
        debug!("[trigger_key_combination] Waiting for all keys to release ({retries})");
        if !Addon::read_context().run_background_thread {
            return;
        }
        thread::sleep(Duration::from_millis(20));
        retries -= 1;
    }

    let mut keys = vec![];
    let mut mouse_buttons = vec![];
    if key_combination.ctrl {
        keys.push(Key::Control);
    }
    if key_combination.shift {
        keys.push(Key::Shift);
    }
    if key_combination.alt {
        keys.push(Key::Alt);
    }
    if key_combination.left_click {
        mouse_buttons.push(Button::Left)
    }
    if let Some(key_code) = &key_combination.key_code {
        keys.push(Key::Other(*key_code));
    } else if let Some(ch) = &key_combination.key {
        keys.push(Key::Unicode(*ch))
    }

    for key in &keys {
        keyboard_key_action(&mut enigo, key, Press)
    }

    for button in &mouse_buttons {
        mouse_button_action(&mut enigo, button, Click);
    }

    thread::sleep(Duration::from_millis(20));

    for key in &keys {
        keyboard_key_action(&mut enigo, key, Release)
    }
}

fn mouse_button_action(enigo: &mut Enigo, button: &Button, action: Direction) {
    match enigo.button(*button, action) {
        Ok(_) => debug!("[mouse_button_action] Mouse press sent: {:?}", button),
        Err(e) => error!("Could not send {:?} {:?} {}", button, action, e),
    }
}

fn keyboard_key_action(enigo: &mut Enigo, key: &Key, action: Direction) {
    match enigo.key(*key, action) {
        Ok(_) => debug!("[keyboard_key_action] Keypress sent: {:?}", key),
        Err(e) => error!("Could not send {:?} {:?} {}", key, action, e),
    }
}
