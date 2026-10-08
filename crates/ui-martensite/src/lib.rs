//! Sovereign retained-mode interface for PdfCraft built on the Martensite GUI engine.

pub mod command_reg;
pub mod menus;
pub mod pdf_view;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use pdfcraft_engine::Engine;
use std::sync::{Arc, Mutex};

/// Application state container managing the Martensite GUI pipeline.
pub struct PdfcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: pdfcraft_engine::Tool,
    pub current_page: usize,
    pub total_pages: usize,
    pub zoom_level: f32,
    pub fit_page: bool,
    pub pan_offset: [f32; 2],
    pub rulers_visible: bool,
    pub sidebar_visible: bool,
    pub is_dirty: bool,
}

impl PdfcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::reader_studio(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: pdfcraft_engine::Tool::Select,
            current_page: 1,
            total_pages: 1,
            zoom_level: 1.0,
            fit_page: false,
            pan_offset: [0.0, 0.0],
            rulers_visible: true,
            sidebar_visible: true,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: pdfcraft_engine::Tool) {
        self.active_tool = tool;
    }

    pub fn goto_page(&mut self, page: usize) {
        self.current_page = page.clamp(1, self.total_pages);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.01, 64.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
        self.fit_page = false;
    }

    pub fn toggle_rulers(&mut self) -> bool {
        self.rulers_visible = !self.rulers_visible;
        self.rulers_visible
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
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = PdfcraftApp::new(engine);
        assert_eq!(app.active_tool, pdfcraft_engine::Tool::Select);
        assert_eq!(app.current_page, 1);
        assert_eq!(app.total_pages, 1);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.rulers_visible);
        assert!(app.sidebar_visible);
        assert!(!app.is_dirty);
    }

    #[test]
    fn test_page_navigation() {
        let engine = Engine::new();
        let mut app = PdfcraftApp::new(engine);
        app.total_pages = 10;
        app.goto_page(5);
        assert_eq!(app.current_page, 5);
        app.goto_page(100);
        assert_eq!(app.current_page, 10);
        app.goto_page(0);
        assert_eq!(app.current_page, 1);
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = PdfcraftApp::new(engine);

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.0001);
        assert_eq!(app.zoom_level, 0.01);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 64.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = PdfcraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.fit_page = true;
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(!app.fit_page);
    }

    #[test]
    fn test_toggles() {
        let engine = Engine::new();
        let mut app = PdfcraftApp::new(engine);

        assert!(app.rulers_visible);
        assert!(!app.toggle_rulers());
        assert!(!app.rulers_visible);
        assert!(app.toggle_rulers());

        assert!(app.sidebar_visible);
        assert!(!app.toggle_sidebar());
        assert!(!app.sidebar_visible);
        assert!(app.toggle_sidebar());
    }
}
