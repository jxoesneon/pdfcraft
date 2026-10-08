//! Options bar widget adapting dynamically to the active tool.

use crate::widgets::scrubby_input::ScrubbyInputWidget;
use pdfcraft_engine::Tool;

pub struct OptionsBarWidget {
    pub active_tool: Tool,
    pub zoom_percent: ScrubbyInputWidget,
    pub rotation: ScrubbyInputWidget,
    pub measure_scale: ScrubbyInputWidget,
    pub redact_code: String,
    pub snapshot_dpi: ScrubbyInputWidget,
}

impl OptionsBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Select,
            zoom_percent: ScrubbyInputWidget::new("Zoom", 100.0, 1.0, 6400.0, "%"),
            rotation: ScrubbyInputWidget::new("Rotate", 0.0, 0.0, 270.0, "°"),
            measure_scale: ScrubbyInputWidget::new("Scale", 1.0, 0.01, 1000.0, "x"),
            redact_code: String::new(),
            snapshot_dpi: ScrubbyInputWidget::new("Resolution", 150.0, 72.0, 2400.0, "dpi"),
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn set_redaction_code(&mut self, code: &str) {
        self.redact_code = code.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_bar_defaults() {
        let mut bar = OptionsBarWidget::new();
        assert_eq!(bar.active_tool, Tool::Select);
        assert_eq!(bar.zoom_percent.value, 100.0);
        assert_eq!(bar.rotation.value, 0.0);
        assert!(bar.redact_code.is_empty());

        bar.set_tool(Tool::Redact);
        assert_eq!(bar.active_tool, Tool::Redact);
        bar.set_redaction_code("FOIA (b)(6)");
        assert_eq!(bar.redact_code, "FOIA (b)(6)");
    }
}
