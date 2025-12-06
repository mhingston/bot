//! Output handling for transcribed text
//!
//! Handles outputting transcribed text as keystrokes or to clipboard.

use super::config::OutputMode;
use crate::error::Result;
use crate::input::Keyboard;

/// Handler for outputting transcribed text
pub struct OutputHandler {
    /// Output mode
    mode: OutputMode,
    /// Keyboard for typing text
    keyboard: Keyboard,
}

impl OutputHandler {
    /// Create a new output handler
    pub fn new(mode: OutputMode) -> Result<Self> {
        let keyboard = Keyboard::new()?;
        Ok(Self { mode, keyboard })
    }

    /// Get the current output mode
    pub fn mode(&self) -> OutputMode {
        self.mode
    }

    /// Set the output mode
    pub fn set_mode(&mut self, mode: OutputMode) {
        self.mode = mode;
    }

    /// Output the transcribed text according to the current mode
    pub fn output(&self, text: &str) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        match self.mode {
            OutputMode::Keystrokes => {
                self.type_text(text)?;
            }
            OutputMode::Clipboard => {
                self.copy_to_clipboard(text)?;
            }
            OutputMode::Both => {
                self.copy_to_clipboard(text)?;
                self.paste()?;
            }
            OutputMode::Logger => {
                // Logger mode - only log, no typing or clipboard
                log::info!("STT Output (Logger): \"{}\"", truncate_for_log(text, 100));
            }
        }

        Ok(())
    }

    /// Type text as keystrokes
    fn type_text(&self, text: &str) -> Result<()> {
        log::info!("Typing text: \"{}\"", truncate_for_log(text, 50));
        self.keyboard.type_string(text)?;
        Ok(())
    }

    /// Copy text to clipboard
    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        log::info!("Copying to clipboard: \"{}\"", truncate_for_log(text, 50));
        crate::clipboard::set_text(text)?;
        Ok(())
    }

    /// Paste from clipboard (Ctrl+V / Cmd+V)
    fn paste(&self) -> Result<()> {
        log::info!("Pasting from clipboard");

        #[cfg(target_os = "macos")]
        {
            // Cmd+V on macOS
            self.keyboard.key_tap("v", Some(&["command".to_string()]))?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Ctrl+V on Windows/Linux
            self.keyboard.key_tap("v", Some(&["control".to_string()]))?;
        }

        Ok(())
    }
}

/// Truncate text for logging
fn truncate_for_log(text: &str, max_len: usize) -> String {
    if text.len() <= max_len { text.to_string() } else { format!("{}...", &text[..max_len]) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_for_log() {
        assert_eq!(truncate_for_log("short", 10), "short");
        assert_eq!(truncate_for_log("this is a long string", 10), "this is a ...");
    }

    #[test]
    fn test_output_mode() {
        // This test requires a display, so just test creation
        if let Ok(handler) = OutputHandler::new(OutputMode::Keystrokes) {
            assert_eq!(handler.mode(), OutputMode::Keystrokes);
        }
    }
}
