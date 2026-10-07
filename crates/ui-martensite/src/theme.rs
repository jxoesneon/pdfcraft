//! PDF viewer theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub app_bg: Color,
    pub paper_bg: Color,
    pub annotation_yellow: Color,
}

impl Theme {
    pub fn reader_studio() -> Self {
        Self {
            app_bg: Color(32, 34, 40),
            paper_bg: Color(255, 255, 255),
            annotation_yellow: Color(255, 235, 120),
        }
    }
}
