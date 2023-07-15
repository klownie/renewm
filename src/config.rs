pub enum Action {
    Close,
    SelectAbove,
    SelectBelow,
    ShrinkFront,
    ExpandFront,
}

pub struct ActionKeyPress {
    pub modifier: u16,
    pub keysym: u32,
    pub action: Action,
}

pub struct Command {
    pub modifier: u16,
    pub keysym: u32,
    pub command: String,
}
pub struct Config {
    pub border_thickness: u32,
    pub border_gap: u32,
    pub active_border: u32,
    pub inactive_border: u32,
    pub workspace_modifier: u16,
    pub workspace_move_window_modifier: u16,
    pub autostart: Vec<String>,
    pub actions: Vec<ActionKeyPress>,
    pub commands: Vec<Command>,
}

pub fn get_config() -> Config {
    todo!()
}
