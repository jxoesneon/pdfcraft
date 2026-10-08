//! Page range widget: split-range selection for Extract Pages and Print ranges.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageRange {
    pub start: usize,
    pub end: usize,
}

impl PageRange {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start).saturating_add(1)
    }

    pub fn is_empty(&self) -> bool {
        self.start > self.end
    }

    pub fn contains(&self, page: usize) -> bool {
        self.start <= page && page <= self.end
    }

    /// Split this range at `at`: returns the (left, right) pair when `at` is an
    /// interior boundary, or `None` when the split would be empty on either side.
    pub fn split_at(&self, at: usize) -> Option<(PageRange, PageRange)> {
        if at <= self.start || at > self.end {
            return None;
        }
        Some((PageRange::new(self.start, at - 1), PageRange::new(at, self.end)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PageRangeWidget {
    pub total_pages: usize,
    pub ranges: Vec<PageRange>,
}

impl PageRangeWidget {
    pub fn new(total_pages: usize) -> Self {
        Self { total_pages, ranges: Vec::new() }
    }

    /// Add a range clamped to the document; out-of-bounds edges are clamped to
    /// the document and ranges that don't intersect it at all are dropped.
    pub fn add_range(&mut self, start: usize, end: usize) {
        if self.total_pages == 0 {
            return;
        }
        let lo = start.min(end);
        let hi = start.max(end);
        if hi < 1 || lo > self.total_pages {
            return;
        }
        self.ranges.push(PageRange::new(lo.max(1), hi.min(self.total_pages)));
    }

    pub fn remove_range(&mut self, idx: usize) {
        if idx < self.ranges.len() {
            self.ranges.remove(idx);
        }
    }

    pub fn selected_pages(&self) -> usize {
        self.ranges.iter().map(|r| r.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_range_split_thresholds() {
        let range = PageRange::new(1, 24);
        assert_eq!(range.len(), 24);
        assert!(range.contains(12));
        assert!(!range.contains(25));

        let (left, right) = range.split_at(10).unwrap();
        assert_eq!(left, PageRange::new(1, 9));
        assert_eq!(right, PageRange::new(10, 24));

        assert!(range.split_at(1).is_none());
        assert!(range.split_at(25).is_none());
    }

    #[test]
    fn test_page_range_widget() {
        let mut widget = PageRangeWidget::new(24);
        widget.add_range(1, 5);
        widget.add_range(20, 100); // Clamped to 20..=24
        widget.add_range(50, 60); // Dropped entirely
        assert_eq!(widget.ranges.len(), 2);
        assert_eq!(widget.selected_pages(), 10);

        widget.remove_range(0);
        assert_eq!(widget.ranges.len(), 1);
        assert_eq!(widget.ranges[0], PageRange::new(20, 24));
    }
}
