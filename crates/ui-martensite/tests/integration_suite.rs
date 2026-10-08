//! Comprehensive integration test suite for PdfCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! scrubby inputs, page ranges, and page coordinates.

use pdfcraft_engine::{Engine, Tool};
use pdfcraft_ui_martensite::{
    PdfcraftApp,
    command_reg::{COMMAND_REGISTRY, find_command},
    menus::generate_main_menu,
    pdf_view::PdfDocState,
    theme::CraftTheme,
    widgets::{
        DockPanelGroup, OptionsBarWidget, OutlineItemDef, OutlineTreeWidget, PageRangeWidget, PageViewWidget, ScrubbyInputWidget, ToolStripWidget,
    },
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = PdfcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Select);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.rulers_visible);
    assert!(app.sidebar_visible);

    // 2. Keystroke Workflow: Switch to EditText, zoom in, hold space to pan
    let new_tool = app.keyboard.on_key_down("e", app.active_tool);
    assert_eq!(new_tool, Some(Tool::EditText));
    app.set_tool(Tool::EditText);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Hand tool
    let hand_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(hand_tool, Some(Tool::Hand));
    app.set_tool(Tool::Hand);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores EditText
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::EditText));
    app.set_tool(Tool::EditText);

    // 3. Options Bar Interaction for Zoom
    let mut options = OptionsBarWidget::new();
    options.set_tool(Tool::Select);
    options.zoom_percent.on_pointer_down(0.0);
    options.zoom_percent.on_pointer_move(20.0, false, false);
    options.zoom_percent.on_pointer_up();
    assert_eq!(options.zoom_percent.value, 120.0); // 100 + 20

    // 4. Outline Tree & Page Navigation
    let mut outline = OutlineTreeWidget::new();
    outline.items.push(OutlineItemDef { id: 1, title: "Cover".to_string(), page: 1, bold: true, italic: false, expanded: false, children: vec![] });
    outline.items.push(OutlineItemDef {
        id: 2,
        title: "Chapter 1".to_string(),
        page: 3,
        bold: false,
        italic: false,
        expanded: true,
        children: vec![],
    });
    outline.select_item(2);
    assert_eq!(outline.selected_item_id, Some(2));
    assert_eq!(outline.current_page, 3);

    // 5. Page Range ("Extract Pages") Selection
    let mut ranges = PageRangeWidget::new(24);
    ranges.add_range(1, 5);
    ranges.add_range(20, 24);
    assert_eq!(ranges.selected_pages(), 10);
    let (left, right) = ranges.ranges[0].split_at(3).unwrap();
    assert_eq!(left.len() + right.len(), 5);

    // 6. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Pages", "Bookmarks", "Comments"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 7. Page View Coordinates
    let mut view = PageViewWidget::new(612, 792);
    view.zoom_at(1.5, [10.0, 10.0]);
    assert!((view.zoom - 1.5).abs() < 1e-4);
    let page_pt = view.screen_to_page([10.0, 10.0]);
    assert!((page_pt[0] - 10.0).abs() < 1e-3);
    assert!((page_pt[1] - 10.0).abs() < 1e-3);

    // 8. Annotation State
    let mut doc = PdfDocState::new();
    doc.add_note(1, [10.0, 10.0, 100.0, 50.0], "Review clause 4");
    assert_eq!(doc.annotations.len(), 1);
    assert_eq!(doc.annotations[0].page, 1);

    // 9. Menu Generation Consistency
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {}", cmd_id);
            }
        }
    }
    assert!(!COMMAND_REGISTRY.is_empty());

    // 10. Theme Color Space Consistency
    let theme = CraftTheme::reader_studio();
    let dark = CraftTheme::dark_neutral();
    assert_ne!(theme.surface_app_bg, dark.surface_app_bg);

    // 11. Tool Strip
    let strip = ToolStripWidget::new();
    assert_eq!(strip.active_tool, Tool::Select);

    // 12. Scrubby clamping sanity
    let mut scrubby = ScrubbyInputWidget::new("Rotate", 0.0, 0.0, 270.0, "°");
    scrubby.set_direct_value(400.0);
    assert_eq!(scrubby.value, 270.0);
}
