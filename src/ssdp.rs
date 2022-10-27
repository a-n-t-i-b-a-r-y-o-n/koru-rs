use std::time::Duration;
use crate::Device;
use std::io::Error;
use async_std::net::UdpSocket;
use regex::Regex;
use std::str::FromStr;
use tokio::time;

/// Parsing and handling of SSDP messages for device discovery

// SSDP response buffer length (bytes)
// Responses _should_ fit within 1024 bytes.
// e.g. My device returns a message longer than the one in the documentation, but still only 267 bytes.
const BUFLEN: usize = 512;

/// Discover Roku devices on the network via SSDP
pub async fn discover_devices(timeout: Duration) -> Result<Vec<Device>, Error> {

    // List of devices
    let mut devices: Vec<Device> = Vec::new();

    // SSDP multicast address
    let address = "239.255.255.250:1900";

    // SSDP discover HTTPU message
    let message = format!(r#"M-SEARCH * HTTP/1.1
Host: {}
Man: "ssdp:discover"
ST: roku:ecp
 "#, address);

    // Create socket
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;

    // Send the multicast message
    socket.send_to(message.as_bytes(), address).await?;

    // Handle responses
    loop {
        // Buffer for received message
        let received: &mut [u8; BUFLEN] = &mut [0u8; BUFLEN];
        // Wait for responses with timeout
        match time::timeout(timeout, socket.recv(received)).await {
            // Handle awaiting response until timeout
            Ok(result) => {
                // Handle receiving response
                match result {
                    Ok(num_bytes) => {
                        // Check if we received the same amount of bytes as the buffer (indicating the buffer probably isn't long enough)
                        if num_bytes == BUFLEN {
                            // TODO: Should we handle handle SSDP responses > 512 bytes? Could they even be from a Roku?
                            println!("[!] WARNING: SSDP message buffer may be too small.")
                        }
                        // If we can parse a Device from the message, push it to the output vec
                        if let Some(device) = handle_ssdp_response(&received[..num_bytes]) {
                            devices.push(device)
                        }

                    }
                    Err(_) => {}
                }
            }
            // Break loop on socket read timeout
            Err(_) => break
        }
    }
    Ok(devices)
}

/// Handler for SSDP responses
fn handle_ssdp_response(raw: &[u8]) -> Option<Device>{
    // Parse message bytes into string, fail silently
    let message = std::str::from_utf8(raw).unwrap_or("");

    // Case-insensitive match for "roku"
    let roku_regex: Regex = Regex::new(r".*[rR]oku.*").unwrap();

    // TODO: Should we validate SSDP responses more thoroughly?
    // Continue only if this response even contains the string "roku"
    if roku_regex.is_match(message) {
        // Continue only if we can parse the location response
        match parse_ssdp_location(message) {
            Some((ipv4, port)) => {
                // Create bare-bones device
                let mut device = Device::new(&ipv4, port);
                // If there's a MAC address in the WAKEUP header, the device support WoL with it
                device.mac_wol = parse_ssdp_mac(message);
                // Return this device
                Some(device)
            }
            None => None
        }
    } else {
        // Couldn't match $roku_regex
        None
    }
}

/// Parse the IP and port number from a LOCATION header in an SSDP response
fn parse_ssdp_location(message: &str) -> Option<(String, i32)> {
    // Regex for IPv4 and port numbers in LOCATION headers
    let location_regex: Regex = Regex::new(r"LOCATION:\shttp://(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d+).+").unwrap();

    // Parse out IP and port, if they exist
    match location_regex.captures(message) {
        Some(location) => Some((String::from(&location[1]), i32::from_str(&location[2]).unwrap_or(8060))),
        None => None
    }
}

/// Parse a MAC address from the WAKEUP header in an SSDP response
fn parse_ssdp_mac(message: &str) -> Option<[u8; 6]> {
    // Regex for MAC addresses in WAKEUP headers
    let wakeup_regex: Regex = Regex::new(r"WAKEUP:.*MAC.(..):(..):(..):(..):(..):(..).*").unwrap();

    // Possible output MAC
    let mut output: [u8; 6] = [0u8; 6];

    // Parse the MAC, if it exists
    if let Some(mac) = wakeup_regex.captures(message) {
        // Parse each address group, failing to zero
        for i in 0..6 {
            // Note that the regex match groups are 1-indexed
            output[i] = u8::from_str_radix(&mac[i+1], 16).unwrap_or(0u8);
        }
        Some(output)
    } else {
        None
    }
}