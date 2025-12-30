use device_query::{DeviceQuery, DeviceState};
use enigo::{
    Button,
    Direction::{self, Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use crate::configuration::keyboard_layout::KeyboardLayout;
use crate::configuration::read_config;
use crate::state::context::read_context;
use log::{debug, error};
use std::thread;
use std::time::Duration;

const AZERTY_A_KEY_CODE: u32 = 81;
const DVORAK_X_KEY_CODE: u32 = 81;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct KeyCombination {
    pub key: Option<Key>,
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
        let key = match read_config().keyboard_layout {
            KeyboardLayout::AZERTY => Key::Other(AZERTY_A_KEY_CODE),
            _ => Key::A,
        };
        KeyCombination {
            key: Some(key),
            ctrl: true,
            ..KeyCombination::default()
        }
    }
    pub fn cut() -> Self {
        let key = match read_config().keyboard_layout {
            KeyboardLayout::DVORAK => Key::Other(DVORAK_X_KEY_CODE),
            _ => Key::X,
        };
        KeyCombination {
            key: Some(key),
            ctrl: true,
            ..KeyCombination::default()
        }
    }
    pub fn enter() -> Self {
        KeyCombination {
            key: Some(Key::Return),
            ..KeyCombination::default()
        }
    }
    pub fn backspace() -> Self {
        KeyCombination {
            key: Some(Key::Backspace),
            ..KeyCombination::default()
        }
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
    let (should_wait_for_key_release, post_key_combination_delay_ms, use_left_shift) = {
        let config = read_config();
        (
            config.wait_until_all_keys_released,
            config.post_key_combination_delay_ms,
            config.use_left_shift,
        )
    };

    while should_wait_for_key_release && retries > 0 {
        if all_keys_released() {
            break;
        }
        debug!("[trigger_key_combination] Waiting for all keys to release ({retries})");
        if !read_context().run_background_thread {
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
        let key = if use_left_shift {
            Key::LShift
        } else {
            Key::RShift
        };
        keys.push(key);
    }
    if key_combination.alt {
        keys.push(Key::Alt);
    }
    if key_combination.left_click {
        mouse_buttons.push(Button::Left)
    }
    if let Some(key) = &key_combination.key {
        keys.push(*key)
    }

    for key in &keys {
        keyboard_key_action(&mut enigo, key, Press)
    }

    for button in &mouse_buttons {
        mouse_button_action(&mut enigo, button, Click);
    }

    thread::sleep(Duration::from_millis(post_key_combination_delay_ms));

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
