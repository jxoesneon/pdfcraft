//! PDF command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "page.rotate_cw", label: "Rotate Clockwise" },
    Command { id: "page.extract", label: "Extract Pages…" },
];
