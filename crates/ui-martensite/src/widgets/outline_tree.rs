//! Bookmark outline tree widget.

#[derive(Clone, Debug, PartialEq)]
pub struct OutlineItemDef {
    pub id: u64,
    pub title: String,
    pub page: usize,
    pub bold: bool,
    pub italic: bool,
    pub expanded: bool,
    pub children: Vec<OutlineItemDef>,
}

pub struct OutlineTreeWidget {
    pub items: Vec<OutlineItemDef>,
    pub selected_item_id: Option<u64>,
    pub current_page: usize,
}

impl OutlineTreeWidget {
    pub fn new() -> Self {
        Self { items: Vec::new(), selected_item_id: None, current_page: 1 }
    }

    pub fn select_item(&mut self, id: u64) {
        self.selected_item_id = Some(id);
        if let Some(item) = find_item(&self.items, id) {
            self.current_page = item.page;
        }
    }

    pub fn toggle_expanded(&mut self, id: u64) {
        if let Some(item) = find_item_mut(&mut self.items, id) {
            item.expanded = !item.expanded;
        }
    }
}

fn find_item(items: &[OutlineItemDef], id: u64) -> Option<&OutlineItemDef> {
    for item in items {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_item(&item.children, id) {
            return Some(found);
        }
    }
    None
}

fn find_item_mut(items: &mut [OutlineItemDef], id: u64) -> Option<&mut OutlineItemDef> {
    for item in items.iter_mut() {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_item_mut(&mut item.children, id) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: u64, title: &str, page: usize, children: Vec<OutlineItemDef>) -> OutlineItemDef {
        OutlineItemDef { id, title: title.to_string(), page, bold: false, italic: false, expanded: false, children }
    }

    #[test]
    fn test_outline_tree_mutation() {
        let mut tree = OutlineTreeWidget::new();
        tree.items.push(item(1, "Chapter 1", 1, vec![item(3, "Section 1.1", 4, vec![])]));
        tree.items.push(item(2, "Chapter 2", 20, vec![]));

        tree.select_item(3);
        assert_eq!(tree.selected_item_id, Some(3));
        assert_eq!(tree.current_page, 4);

        tree.toggle_expanded(1);
        assert!(tree.items[0].expanded);
        tree.toggle_expanded(1);
        assert!(!tree.items[0].expanded);
    }
}
