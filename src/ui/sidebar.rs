#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Scan,
    Quarantine,
    Settings,
    About,
}

impl Tab {
    pub fn icon(&self) -> &'static str {
        match self {
            Tab::Scan => "🔍",
            Tab::Quarantine => "🛡️",
            Tab::Settings => "⚙️",
            Tab::About => "ℹ️",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Tab::Scan => "Scan",
            Tab::Quarantine => "Quarantine",
            Tab::Settings => "Settings",
            Tab::About => "About",
        }
    }
}
