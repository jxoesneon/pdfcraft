//! Tool strip widget: single/double column layout and the active document tool.

use pdfcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Select, alternatives: &[] },
    ToolSlot { primary: Tool::Hand, alternatives: &[] },
    ToolSlot { primary: Tool::Snapshot, alternatives: &[] },
    ToolSlot { primary: Tool::Zoom, alternatives: &[] },
    ToolSlot { primary: Tool::EditText, alternatives: &[] },
    ToolSlot { primary: Tool::Comment, alternatives: &[] },
    ToolSlot { primary: Tool::Stamp, alternatives: &[] },
    ToolSlot { primary: Tool::FillSign, alternatives: &[] },
    ToolSlot { primary: Tool::Measure, alternatives: &[] },
    ToolSlot { primary: Tool::Redact, alternatives: &[] },
    ToolSlot { primary: Tool::CropPages, alternatives: &[] },
    ToolSlot { primary: Tool::OrganizePages, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub double_column: bool,
    pub sidebar_visible: bool,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self { active_tool: Tool::Select, double_column: false, sidebar_visible: true }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    pub fn toggle_sidebar(&mut self) -> bool {
        self.sidebar_visible = !self.sidebar_visible;
        self.sidebar_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Select);
        assert!(!strip.double_column);
        assert!(strip.sidebar_visible);

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());

        assert!(!strip.toggle_sidebar());
        assert!(!strip.sidebar_visible);
    }
}
