
use wake_on_lan::MagicPacket;

use crate::{Button, Device};

/// Possible power states for a device to be in
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PowerState {
    Off,        // Powered down, requires wake-on-lan
    DisplayOff, // Screen off, hardware on, still accessible via API
    On,         // Screen is on
    Unknown,    // ???
}

impl From<&str> for PowerState {
    fn from(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "off" | "poweroff" => PowerState::Off,
            "displayoff" => PowerState::DisplayOff,
            "on" | "poweron" => PowerState::On,
            _ => PowerState::Unknown
        }
    }
}

impl From<String> for PowerState { fn from(s: String) -> Self { PowerState::from(s.as_str()) } }

impl ToString for PowerState {
    fn to_string(&self) -> String {
        match self {
            PowerState::Off => String::from("Off"),
            PowerState::DisplayOff => String::from("DisplayOff"),
            PowerState::On => String::from("On"),
            PowerState::Unknown => String::from("Unknown"),
        }
    }
}

impl Device {
    /// Whether the device support Wake-on-LAN
    pub fn supports_wake_on_lan(&self) -> bool {
        match self.mac_wol {
            None => false,
            Some(_) => true,
        }
    }

    /// Toggle the device power state, sending a Wake-on-LAN packet if required
    pub async fn toggle_power_state(&mut self) {
        match self.power_state {
            PowerState::Off => {
                if !self.supports_wake_on_lan() {
                    println!("[!] Device is off but doesn't support Wake-on-LAN");
                    return;
                }

                if let Err(e) = MagicPacket::new(&self.mac_wol.unwrap()).send() {
                    println!("[!] Unable to send Wake-on-LAN: {:?}", e);
                }
            }
            _ => {
                let _ = self.press_button(Button::Power).await;
            }
        }
    }
}