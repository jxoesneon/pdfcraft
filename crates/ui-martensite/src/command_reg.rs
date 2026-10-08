//! Decoupled command catalog and taxonomy for PdfCraft.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Document,
    Pages,
    Comment,
    View,
    Window,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec { id: "file.new", label: "New…", category: CommandCategory::File, default_shortcut: Some("Cmd+N"), secondary_shortcut: None },
    CommandSpec { id: "file.open", label: "Open…", category: CommandCategory::File, default_shortcut: Some("Cmd+O"), secondary_shortcut: None },
    CommandSpec { id: "file.save", label: "Save", category: CommandCategory::File, default_shortcut: Some("Cmd+S"), secondary_shortcut: None },
    CommandSpec {
        id: "file.save_as",
        label: "Save As…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+S"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "file.export_pdf",
        label: "Export PDF…",
        category: CommandCategory::File,
        default_shortcut: Some("Alt+Shift+Cmd+E"),
        secondary_shortcut: None,
    },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Shift+Cmd+Z"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.select_all",
        label: "Select All",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+A"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "edit.find", label: "Find…", category: CommandCategory::Edit, default_shortcut: Some("Cmd+F"), secondary_shortcut: None },
    // Document
    CommandSpec {
        id: "doc.properties",
        label: "Document Properties…",
        category: CommandCategory::Document,
        default_shortcut: Some("Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "doc.reduce_size",
        label: "Reduce File Size…",
        category: CommandCategory::Document,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // Pages
    CommandSpec {
        id: "page.rotate_cw",
        label: "Rotate Clockwise",
        category: CommandCategory::Pages,
        default_shortcut: Some("Shift+Cmd+R"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "page.extract", label: "Extract Pages…", category: CommandCategory::Pages, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "page.insert", label: "Insert Pages…", category: CommandCategory::Pages, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "page.crop", label: "Crop Pages…", category: CommandCategory::Pages, default_shortcut: None, secondary_shortcut: None },
    // Comment
    CommandSpec {
        id: "comment.note",
        label: "Add Sticky Note",
        category: CommandCategory::Comment,
        default_shortcut: Some("Cmd+6"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "comment.highlight",
        label: "Highlight Text",
        category: CommandCategory::Comment,
        default_shortcut: Some("Cmd+7"),
        secondary_shortcut: None,
    },
    // View
    CommandSpec {
        id: "view.fit_page",
        label: "Fit Page",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+0"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.actual",
        label: "Actual Size",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+1"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.sidebar",
        label: "Navigation Pane",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+B"),
        secondary_shortcut: Some("F4"),
    },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.map(|c| c.label), Some(cmd.label));
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Pages).is_empty());
        assert!(!commands_by_category(CommandCategory::Comment).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }
}
