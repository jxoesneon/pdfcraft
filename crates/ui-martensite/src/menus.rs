//! PDF menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "Page", items: &["page.rotate_cw", "page.extract"] },
];
