//! PDF integration tests.

use pdfcraft_ui_martensite::PdfcraftApp;

#[test]
fn test_pdf_workflow() {
    let mut app = PdfcraftApp::new();
    app.total_pages = 24;
    app.goto_page(12);
    assert_eq!(app.current_page, 12);
}
