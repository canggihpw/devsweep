#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Scan,
    Largest,
    Quarantine,
    Settings,
    About,
}

impl Tab {
    pub fn icon(&self) -> &'static str {
        match self {
            Tab::Scan => "🔍",
            Tab::Largest => "📊",
            Tab::Quarantine => "🛡️",
            Tab::Settings => "⚙️",
            Tab::About => "ℹ️",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Tab::Scan => "Scan",
            Tab::Largest => "Largest",
            Tab::Quarantine => "Quarantine",
            Tab::Settings => "Settings",
            Tab::About => "About",
        }
    }
}
