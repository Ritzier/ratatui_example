#[derive(Default)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}

impl AppState {
    pub fn is_running(&self) -> bool {
        match self {
            Self::Running => true,
            Self::Quitting => false,
        }
    }
}
