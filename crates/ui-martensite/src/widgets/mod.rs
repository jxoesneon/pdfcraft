//! Martensite widget suite for PdfCraft.

pub mod dock_panel;
pub mod options_bar;
pub mod outline_tree;
pub mod page_range;
pub mod page_view;
pub mod scrubby_input;
pub mod tool_strip;

pub use dock_panel::DockPanelGroup;
pub use options_bar::OptionsBarWidget;
pub use outline_tree::{OutlineItemDef, OutlineTreeWidget};
pub use page_range::{PageRange, PageRangeWidget};
pub use page_view::PageViewWidget;
pub use scrubby_input::ScrubbyInputWidget;
pub use tool_strip::ToolStripWidget;
