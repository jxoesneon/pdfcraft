//! Keystroke state machine providing 100% Acrobat keyboard ergonomics.

use pdfcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Hand)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Standard Acrobat single-key shortcuts
            "v" | "V" => Some(Tool::Select),
            "h" | "H" => Some(Tool::Hand),
            "s" | "S" => Some(Tool::Snapshot),
            "e" | "E" => Some(Tool::EditText),
            "n" | "N" => Some(Tool::Comment),
            "k" | "K" => Some(Tool::Stamp),
            "f" | "F" => Some(Tool::FillSign),
            "m" | "M" => Some(Tool::Measure),
            "r" | "R" => Some(Tool::Redact),
            "c" | "C" => Some(Tool::CropPages),
            "o" | "O" => Some(Tool::OrganizePages),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", Tool::EditText), Some(Tool::Select));
        assert_eq!(k.on_key_down("E", Tool::Select), Some(Tool::EditText));
        assert_eq!(k.on_key_down("s", Tool::EditText), Some(Tool::Snapshot));
        assert_eq!(k.on_key_down("n", Tool::Snapshot), Some(Tool::Comment));
        assert_eq!(k.on_key_down("k", Tool::Comment), Some(Tool::Stamp));
        assert_eq!(k.on_key_down("f", Tool::Stamp), Some(Tool::FillSign));
        assert_eq!(k.on_key_down("m", Tool::FillSign), Some(Tool::Measure));
        assert_eq!(k.on_key_down("r", Tool::Measure), Some(Tool::Redact));
        assert_eq!(k.on_key_down("c", Tool::Redact), Some(Tool::CropPages));
        assert_eq!(k.on_key_down("o", Tool::CropPages), Some(Tool::OrganizePages));
    }

    #[test]
    fn test_spring_loaded_hand_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Select;

        // Press Space: temporary Hand
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Hand));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Hand), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Comment;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}
