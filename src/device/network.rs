/// Network types a device could be connected to
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkType {
    Wireless,   // e.g. Wi-Fi
    Ethernet,   // Ethernet cable
}

impl ToString for NetworkType {
    fn to_string(&self) -> String {
        match self {
            NetworkType::Wireless => String::from("Wireless"),
            NetworkType::Ethernet => String::from("Ethernet")
        }
    }
}

impl From<&str> for NetworkType {
    fn from(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "ethernet" => NetworkType::Ethernet,
            _ => NetworkType::Wireless,
        }
    }
}

impl From<String> for NetworkType { fn from(s: String) -> Self { NetworkType::from(s.as_str()) } }