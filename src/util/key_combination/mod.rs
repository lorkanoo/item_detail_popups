use crate::addon::Addon;
use device_query::{DeviceQuery, DeviceState};
use enigo::{
    Button, Direction,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use function_name::named;
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
            ..Default::default()
        }
    }
    pub fn select_all() -> Self {
        KeyCombination {
            key: Some('a'),
            ctrl: true,
            ..Default::default()
        }
    }
    pub fn cut() -> Self {
        KeyCombination {
            key: Some('x'),
            ctrl: true,
            ..Default::default()
        }
    }
    pub fn enter() -> Self {
        KeyCombination {
            key_code: Some(13),
            ..Default::default()
        }
    }
    pub fn backspace() -> Self {
        KeyCombination {
            key_code: Some(8),
            ..Default::default()
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
    keys.is_empty()
}

pub fn trigger_key_combination(key_combination: &KeyCombination) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    loop {
        if all_keys_released() {
            break;
        }
        debug!("Waiting for all keys to release..");
        if !Addon::lock().context.run_background_thread {
            return;
        }
        thread::sleep(Duration::from_millis(50));
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

    thread::sleep(Duration::from_millis(40));

    for key in &keys {
        keyboard_key_action(&mut enigo, key, Release)
    }
}

#[named]
fn mouse_button_action(enigo: &mut Enigo, button: &Button, action: Direction) {
    match enigo.button(*button, action) {
        Ok(_) => debug!("[{}] Mouse press sent", function_name!()),
        Err(_) => error!("Could not send {:?} {:?}", button, action),
    }
}

#[named]
fn keyboard_key_action(enigo: &mut Enigo, key: &Key, action: Direction) {
    match enigo.key(*key, action) {
        Ok(_) => debug!("[{}] Keypress sent", function_name!()),
        Err(_) => error!("Could not send {:?} {:?}", key, action),
    }
}
