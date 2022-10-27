mod network;
mod power;

use ecp::{ContentData, Get, Request, Response, Set};

pub use crate::device::network::NetworkType;
pub use crate::device::power::PowerState;

use ecp::Connection;
use std::collections::HashMap;
use quick_xml::{Reader, events::Event};
use crate::App;
use std::ops::Deref;
use std::str::FromStr;

/// Device object
#[derive(Clone, Debug)]
pub struct Device {
    pub connection:     Option<Connection>, // ECP-2 connection
    pub ipv4:           String,             // IPv4 address
    pub port:           i32,                // Port (Default: 8060)
    pub name:           String,             // Device name
    pub network:        NetworkType,        // Connected network type
    pub mac_wol:        Option<[u8; 6]>,    // MAC address used for Wake-on-LAN (if supported)
    pub mac_wlan:       Option<[u8; 6]>,    // MAC address for WLAN
    pub mac_eth:        Option<[u8; 6]>,    // MAC address for Ethernet
    pub power_state:    PowerState,         // Last-known device power state
}

impl Device {
    /// Constructor w/ only IPv4 and port
    pub fn new(ipv4: &str, port: i32) -> Device {
        Device {
            connection: None,
            ipv4: String::from(ipv4),
            port,
            name: "".to_string(),
            network: NetworkType::Wireless,
            mac_wol: None,
            mac_wlan: None,
            mac_eth: None,
            power_state: PowerState::Unknown,
        }
    }

    /// Open an ECP-2 connection to the device and authenticate, returning auth result
    pub async fn connect(&mut self, key: Vec<u8>) -> bool {
        let octets: Vec<&str> = self.ipv4.split('.').collect();
        let ipv4: [u8; 4] = [
            octets[0].parse::<u8>().unwrap_or(0),
            octets[1].parse::<u8>().unwrap_or(0),
            octets[2].parse::<u8>().unwrap_or(0),
            octets[3].parse::<u8>().unwrap_or(0),
        ];
        let mut connection = Connection::new(ipv4, key);
        connection.open().await;

        if connection.is_authenticated() {
            self.connection = Some(connection);
        }

        match self.connection {
            None => false,
            Some(_) => true,
        }
    }

    /// Whether the connection has been opened
    pub fn is_connected(&self) -> bool {
        match self.connection {
            None => false,
            Some(_) => true,
        }
    }

    /// Send an arbitrary request to the device and return the response
    pub async fn send_request(&mut self, request: Request) -> Option<Response> {
        let request_id = self.next_sync_number();
        if let Some(connection) = &mut self.connection {
            let message = request.set_request_id(request_id);
            connection.send_request(message).await
        }
        else {
            None
        }
    }

    /// Return parsed device-info XML
    pub async fn get_info(&mut self) -> Result<HashMap<String, String>, String> {
        if !self.is_connected() {
            return Err(String::from("Not connected to device"));
        }

        if let Some(message) = self.send_request(Get::DeviceInfo.into()).await {
            if let Some(ContentData::Text { string: xml }) = message.content_data {
                // Parsed XML keys/values
                let mut xml_parsed: HashMap<String, String> = HashMap::new();
                // Create XML reader
                let mut reader = Reader::from_str(&xml);
                reader.trim_text(true);
                // XML event buffer
                let mut buffer = Vec::new();
                // Current tag
                let mut tag = String::new();
                // Loop the XML
                loop {
                    match reader.read_event(&mut buffer) {
                        // Read each tag
                        Ok(Event::Start(ref e)) => tag = std::str::from_utf8(e.name()).unwrap_or("").to_string(),
                        // Handle tag content
                        Ok(Event::Text(e)) => {
                            // Skip working with top-level tags
                            if tag != "?xml" && tag != "device-info" {
                                // Create new entry in hashmap
                                xml_parsed.insert(
                                    tag.clone(),
                                    e.unescape_and_decode(&reader).unwrap_or(String::new())
                                );
                            }
                        },
                        // Break at EOF
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buffer.clear();
                }
                // Return hashmap of xml
                return Ok(xml_parsed);
            }

            return Err(String::from("Response had no content"))
        }

        Err(String::from("Empty response received"))
    }

    /// Get list of installed apps
    pub async fn get_installed_apps(&mut self) -> Result<Vec<App>, String> {
        if !self.is_connected() {
            return Err(String::from("Not connected to device"));
        }

        if let Some(message) = self.send_request(Get::InstalledApps.into()).await {
            if let Some(ContentData::Text { string: xml }) = message.content_data {
                // Parsed XML keys/values
                let mut apps_parsed: Vec<App> = Vec::new();
                // Create XML reader
                let mut reader = Reader::from_str(&xml);
                reader.trim_text(true);
                // XML event buffer
                let mut buffer = Vec::new();
                // Whether to read tag content
                let mut read = false;
                // Current roku app from tag
                let mut app = App {
                    id: 0,
                    apptype: "".to_string(),
                    version: "".to_string(),
                    name: "".to_string(),
                    icon: None
                };
                // Loop the XML
                loop {
                    match reader.read_event(&mut buffer) {
                        // Read each tag
                        Ok(Event::Start(ref e)) => {
                            if e.name() != b"?xml" && e.name() != b"apps" {
                                // Parse and collect attributes
                                let attributes = e.attributes()
                                    .map(|a| a.unwrap().value)
                                    .collect::<Vec<_>>();
                                // Create RokuApp object from attributes
                                app = App {
                                    id: i32::from_str(&std::str::from_utf8(attributes[0].deref()).unwrap_or("").to_string()).unwrap(),
                                    apptype: std::str::from_utf8(attributes[1].deref()).unwrap_or("").to_string(),
                                    version: std::str::from_utf8(attributes[2].deref()).unwrap_or("").to_string(),
                                    name: String::new(),
                                    icon: None
                                };
                                // Prepare to read tag content
                                read = true;
                            }
                        },
                        // Handle tag content
                        Ok(Event::Text(e)) => {
                            // Skip working with top-level tags
                            if read {
                                // Update currently-parsed app name
                                app.name = e.unescape_and_decode(&reader)
                                    .unwrap_or(String::new())
                                    .replace("\u{a0}", "");     // There are newline characters in some names
                                // Add app to list of parsed apps
                                apps_parsed.push(app.clone());
                            }
                        },
                        // Break at EOF
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buffer.clear();
                }
                // Return list of apps
                return Ok(apps_parsed)
            }

            return Err(String::from("Response had no content"));
        }

        Err(String::from("Empty response received"))
    }

    /// Launch an app by its id
    pub async fn launch_app_by_id(&mut self, app_id: i32) {
        self.send_request(Set::LaunchApp { channel_id: app_id }.into()).await;
    }

    /// Manually update this object to match real-world device
    pub async fn update_self(&mut self) {
        // Attempt to get complete device info (we currently only have IP & port)
        if let Ok(info) = self.get_info().await {
            // Update device object with new info using the hashmap
            self.name = info.get("friendly-device-name").unwrap().clone();
            self.network = NetworkType::from(info.get("network-type").unwrap().clone().to_ascii_uppercase());
            self.mac_wlan = Some(split_mac(info.get("wifi-mac").unwrap()));
            // Handle failing to resolve this from the hashmap (do devices w/o support still have it?)
            if let Some(support) = info.get("supports-ethernet") {
                // Check if this device supports ethernet
                if support.to_ascii_uppercase().as_str() == "TRUE" {
                    // Parse the Ethernet MAC
                    self.mac_eth = Some(split_mac(info.get("ethernet-mac").unwrap_or(&"0:0:0:0:0:0".to_string())));
                }
            }
        }
    }

    /// Get the next message sync number
    fn next_sync_number(&mut self) -> i32 {
        if let Some(connection) = &mut self.connection {
            connection.next_sync_number()
        }
        else {
            0
        }
    }
}

/// Split up device MACs into byte arrays
fn split_mac(input: &str) -> [u8; 6] {
    let mut index = 0;
    let mut output: [u8; 6] = [0; 6];
    str::split(input, ':')
        .for_each(|chunk| {
            output[index] = u8::from_str_radix(chunk, 16).unwrap();
            index += 1;
        });
    output
}