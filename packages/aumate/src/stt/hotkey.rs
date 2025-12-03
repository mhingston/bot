//! Global hotkey management using rdev
//!
//! Provides cross-platform global hotkey registration for STT.

use super::config::{HotkeyConfig, HotkeyMode, Modifier};
use crate::error::{AumateError, Result};
use rdev::{Event, EventType, Key, listen};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Hotkey event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    /// Start recording
    RecordStart,
    /// Stop recording
    RecordStop,
}

/// Callback type for hotkey events
pub type HotkeyCallback = Arc<dyn Fn(HotkeyEvent) + Send + Sync>;

/// Global hotkey manager
pub struct HotkeyManager {
    /// Whether the listener is running
    is_running: Arc<AtomicBool>,
    /// Listener thread handle
    listener_handle: Option<JoinHandle<()>>,
    /// Currently pressed modifier keys
    pressed_modifiers: Arc<Mutex<HashSet<Modifier>>>,
    /// Whether the main key is pressed
    main_key_pressed: Arc<AtomicBool>,
    /// Whether we're in recording state (for toggle mode)
    is_recording: Arc<AtomicBool>,
    /// Hotkey configuration
    config: Arc<Mutex<HotkeyConfig>>,
    /// Event callback
    callback: Option<HotkeyCallback>,
}

impl HotkeyManager {
    /// Create a new hotkey manager
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            listener_handle: None,
            pressed_modifiers: Arc::new(Mutex::new(HashSet::new())),
            main_key_pressed: Arc::new(AtomicBool::new(false)),
            is_recording: Arc::new(AtomicBool::new(false)),
            config: Arc::new(Mutex::new(HotkeyConfig::default())),
            callback: None,
        }
    }

    /// Set the hotkey configuration
    pub fn set_config(&mut self, config: HotkeyConfig) {
        *self.config.lock().unwrap() = config;
    }

    /// Get the current hotkey configuration
    pub fn config(&self) -> HotkeyConfig {
        self.config.lock().unwrap().clone()
    }

    /// Set the event callback
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(HotkeyEvent) + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback));
    }

    /// Check if the listener is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    /// Start listening for hotkeys
    pub fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        let callback = self
            .callback
            .clone()
            .ok_or_else(|| AumateError::Other("No callback set".to_string()))?;

        let is_running = self.is_running.clone();
        let pressed_modifiers = self.pressed_modifiers.clone();
        let main_key_pressed = self.main_key_pressed.clone();
        let is_recording = self.is_recording.clone();
        let config = self.config.clone();

        is_running.store(true, Ordering::Relaxed);

        let handle = thread::spawn(move || {
            let _ = listen(move |event: Event| {
                if !is_running.load(Ordering::Relaxed) {
                    return;
                }

                let config = config.lock().unwrap();
                let target_key = parse_key(&config.key);

                match event.event_type {
                    EventType::KeyPress(key) => {
                        // Track modifier keys
                        if let Some(modifier) = key_to_modifier(&key) {
                            pressed_modifiers.lock().unwrap().insert(modifier);
                        }

                        // Check if this is our target key
                        if Some(key) == target_key {
                            // Check if all required modifiers are pressed
                            let pressed = pressed_modifiers.lock().unwrap();
                            let all_modifiers_pressed =
                                config.modifiers.iter().all(|m| pressed.contains(m));

                            if all_modifiers_pressed && !main_key_pressed.load(Ordering::Relaxed) {
                                main_key_pressed.store(true, Ordering::Relaxed);

                                match config.mode {
                                    HotkeyMode::PushToTalk => {
                                        // Start recording on key press
                                        callback(HotkeyEvent::RecordStart);
                                    }
                                    HotkeyMode::Toggle => {
                                        // Toggle recording state
                                        let was_recording =
                                            is_recording.fetch_xor(true, Ordering::Relaxed);
                                        if was_recording {
                                            callback(HotkeyEvent::RecordStop);
                                        } else {
                                            callback(HotkeyEvent::RecordStart);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        // Track modifier keys
                        if let Some(modifier) = key_to_modifier(&key) {
                            pressed_modifiers.lock().unwrap().remove(&modifier);
                        }

                        // Check if this is our target key
                        if Some(key) == target_key && main_key_pressed.load(Ordering::Relaxed) {
                            main_key_pressed.store(false, Ordering::Relaxed);

                            if config.mode == HotkeyMode::PushToTalk {
                                // Stop recording on key release
                                callback(HotkeyEvent::RecordStop);
                            }
                        }
                    }
                    _ => {}
                }
            });
        });

        self.listener_handle = Some(handle);
        log::info!("Hotkey listener started");

        Ok(())
    }

    /// Stop listening for hotkeys
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        self.is_recording.store(false, Ordering::Relaxed);
        self.main_key_pressed.store(false, Ordering::Relaxed);
        self.pressed_modifiers.lock().unwrap().clear();

        // Note: The rdev listener thread will exit on next event
        // We can't join it directly as listen() is blocking
        self.listener_handle = None;

        log::info!("Hotkey listener stopped");
    }

    /// Reset recording state (for toggle mode)
    pub fn reset_recording_state(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Convert a key string to rdev Key
fn parse_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "space" => Some(Key::Space),
        "enter" | "return" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "escape" | "esc" => Some(Key::Escape),
        "backspace" => Some(Key::Backspace),
        "delete" | "del" => Some(Key::Delete),
        "up" => Some(Key::UpArrow),
        "down" => Some(Key::DownArrow),
        "left" => Some(Key::LeftArrow),
        "right" => Some(Key::RightArrow),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" => Some(Key::PageUp),
        "pagedown" => Some(Key::PageDown),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        "a" => Some(Key::KeyA),
        "b" => Some(Key::KeyB),
        "c" => Some(Key::KeyC),
        "d" => Some(Key::KeyD),
        "e" => Some(Key::KeyE),
        "f" => Some(Key::KeyF),
        "g" => Some(Key::KeyG),
        "h" => Some(Key::KeyH),
        "i" => Some(Key::KeyI),
        "j" => Some(Key::KeyJ),
        "k" => Some(Key::KeyK),
        "l" => Some(Key::KeyL),
        "m" => Some(Key::KeyM),
        "n" => Some(Key::KeyN),
        "o" => Some(Key::KeyO),
        "p" => Some(Key::KeyP),
        "q" => Some(Key::KeyQ),
        "r" => Some(Key::KeyR),
        "s" => Some(Key::KeyS),
        "t" => Some(Key::KeyT),
        "u" => Some(Key::KeyU),
        "v" => Some(Key::KeyV),
        "w" => Some(Key::KeyW),
        "x" => Some(Key::KeyX),
        "y" => Some(Key::KeyY),
        "z" => Some(Key::KeyZ),
        "0" => Some(Key::Num0),
        "1" => Some(Key::Num1),
        "2" => Some(Key::Num2),
        "3" => Some(Key::Num3),
        "4" => Some(Key::Num4),
        "5" => Some(Key::Num5),
        "6" => Some(Key::Num6),
        "7" => Some(Key::Num7),
        "8" => Some(Key::Num8),
        "9" => Some(Key::Num9),
        _ => None,
    }
}

/// Convert an rdev Key to our Modifier type
fn key_to_modifier(key: &Key) -> Option<Modifier> {
    match key {
        Key::ControlLeft | Key::ControlRight => Some(Modifier::Ctrl),
        Key::Alt | Key::AltGr => Some(Modifier::Alt),
        Key::ShiftLeft | Key::ShiftRight => Some(Modifier::Shift),
        Key::MetaLeft | Key::MetaRight => Some(Modifier::Meta),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key() {
        assert_eq!(parse_key("Space"), Some(Key::Space));
        assert_eq!(parse_key("space"), Some(Key::Space));
        assert_eq!(parse_key("F1"), Some(Key::F1));
        assert_eq!(parse_key("a"), Some(Key::KeyA));
        assert_eq!(parse_key("unknown"), None);
    }

    #[test]
    fn test_key_to_modifier() {
        assert_eq!(key_to_modifier(&Key::ControlLeft), Some(Modifier::Ctrl));
        assert_eq!(key_to_modifier(&Key::ShiftLeft), Some(Modifier::Shift));
        assert_eq!(key_to_modifier(&Key::Alt), Some(Modifier::Alt));
        assert_eq!(key_to_modifier(&Key::MetaLeft), Some(Modifier::Meta));
        assert_eq!(key_to_modifier(&Key::Space), None);
    }

    #[test]
    fn test_hotkey_manager_creation() {
        let manager = HotkeyManager::new();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_config_update() {
        let mut manager = HotkeyManager::new();
        let config = HotkeyConfig {
            key: "F1".to_string(),
            modifiers: vec![Modifier::Ctrl],
            mode: HotkeyMode::Toggle,
        };
        manager.set_config(config.clone());
        assert_eq!(manager.config().key, "F1");
    }
}
