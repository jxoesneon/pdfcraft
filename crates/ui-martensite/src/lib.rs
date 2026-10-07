//! Sovereign retained-mode PDF interface for PDFCraft built on Martensite.

pub mod command_reg;
pub mod menus;
pub mod pdf_view;
pub mod theme;

pub struct PdfcraftApp {
    pub current_page: usize,
    pub total_pages: usize,
    pub zoom: f32,
    pub fit_page: bool,
}

impl PdfcraftApp {
    pub fn new() -> Self {
        Self {
            current_page: 1,
            total_pages: 1,
            zoom: 1.0,
            fit_page: false,
        }
    }

    pub fn goto_page(&mut self, page: usize) {
        self.current_page = page.clamp(1, self.total_pages);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdfcraft_app() {
        let mut app = PdfcraftApp::new();
        app.total_pages = 10;
        app.goto_page(5);
        assert_eq!(app.current_page, 5);
        app.goto_page(100);
        assert_eq!(app.current_page, 10);
    }
}
