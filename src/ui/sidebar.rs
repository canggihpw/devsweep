#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Scan,
    Ports,
    Trends,
    Quarantine,
    Settings,
    About,
}

impl Tab {
    pub fn icon(&self) -> &'static str {
        match self {
            Tab::Scan => "🔍",
            Tab::Ports => "🔌",
            Tab::Trends => "📊",
            Tab::Quarantine => "🛡️",
            Tab::Settings => "⚙️",
            Tab::About => "ℹ️",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Tab::Scan => "Scan",
            Tab::Ports => "Ports",
            Tab::Trends => "Trends",
            Tab::Quarantine => "Quarantine",
            Tab::Settings => "Settings",
            Tab::About => "About",
        }
    }
}
