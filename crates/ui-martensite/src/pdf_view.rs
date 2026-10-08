//! PDF page and annotation models.

pub struct Annotation {
    pub page: usize,
    pub rect: [f32; 4],
    pub contents: String,
}

pub struct PdfDocState {
    pub annotations: Vec<Annotation>,
}

impl PdfDocState {
    pub fn new() -> Self {
        Self { annotations: Vec::new() }
    }

    pub fn add_note(&mut self, page: usize, rect: [f32; 4], text: &str) {
        self.annotations.push(Annotation { page, rect, contents: text.to_string() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotations() {
        let mut doc = PdfDocState::new();
        doc.add_note(1, [10.0, 10.0, 100.0, 50.0], "Review clause 4");
        assert_eq!(doc.annotations.len(), 1);
    }
}
