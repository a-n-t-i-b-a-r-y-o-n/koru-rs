
mod app;
mod remote;
mod device;
mod ssdp;
mod config;

// Re-export higher-level stuff
pub use crate::app::*;
pub use crate::remote::*;
pub use crate::device::*;
pub use crate::ssdp::discover_devices;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn download_all_icons() {
        let output_dir = Path::new("/var/tmp/");
        match &mut discover_devices(Duration::new(10, 0)).await {
            Ok(devices) => {
                for device in devices {
                    device.connect(load_ecp2_key()).await;
                    match device.get_installed_apps().await {
                        Ok(apps) => {
                            for mut app in apps {
                                app.fetch_icon(device).await;
                                let filename = format!("{}.png", app.id);
                                match File::create(output_dir.join(Path::new(filename.as_str()))) {
                                    Ok(mut file) => {
                                        match file.write_all(app.icon.unwrap().as_slice()) {
                                            Ok(_) => {
                                                println!("[-] Wrote: {}", filename);
                                            }
                                            Err(e) => {
                                                println!("[!] Error writing \"{}\": {:?}", filename, e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("[!] {}", e.to_string());
                                        assert!(false);
                                    }
                                }
                            }
                        }
                        Err(message) => {
                            println!("[!] {}", message);
                            assert!(false);
                        }
                    }
                }
            }
            Err(_) => {
                println!("[!] {}", "No devices found");
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn get_all_apps() {
        match &mut discover_devices(Duration::new(10, 0)).await {
            Ok(devices) => {
                for device in devices {
                    println!("## Device: {}", device.ipv4);
                    device.connect(load_ecp2_key()).await;
                    match device.get_installed_apps().await {
                        Ok(apps) => {
                            for app in apps {
                                println!("App: {}", app.id);
                            }
                        }
                        Err(message) => {
                            println!("{}", message);
                            assert!(false);
                        }
                    }
                }
            }
            Err(_) => {
                assert!(false, "No devices found")
            }
        }
    }

    #[allow(dead_code)]
    fn load_ecp2_key() -> Vec<u8> {
        let config = config::load_from_file("conf/secrets");
        assert!(config.len() > 0);
        assert!(config.contains_key("ecp2_key"));
        let key = config.get("ecp2_key").unwrap().as_bytes().to_vec();
        key
    }

    #[tokio::test]
    async fn try_discover_devices() {
        println!("[-] Attempting device discovery...");
        match discover_devices(Duration::new(5, 0)).await {
            Ok(devices) => {
                assert_ne!(devices.len(), 0);
                // Print the names of the discovered devices
                println!("[+] Discovered devices:\n------------------------------");
                for device in devices.iter() {
                    println!("IP: {}", device.ipv4);
                    println!("Port: {}", device.port);
                    println!("MAC: {:02x?} / {:02x?}", device.mac_eth, device.mac_wlan);
                }
                println!("------------------------------");
            }
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn connect_to_discovered_device() {
        // Try for 5s to discover devices
        match &mut discover_devices(Duration::new(5, 0)).await {
            Ok(devices) => {
                // Assert we found a device
                assert!(devices.len() > 0, "Failed to discover any devices");

                // Work with the first device
                let mut device = devices.pop().unwrap();
                println!("[-] Device: {}", device.ipv4);
                assert_eq!(device.is_connected(), false);

                // Establish an ECP-2 connection with the device
                device.connect(load_ecp2_key()).await;
                assert_eq!(device.is_connected(), true);
            }
            Err(_) => {}
        }
    }

    #[tokio::test]
    async fn get_discovered_device_info() {
        // Try for 5s to discover devices
        match &mut discover_devices(Duration::new(5, 0)).await {
            Ok(devices) => {
                // Assert we found a device
                assert!(devices.len() > 0, "Failed to discover any devices");

                // Work with the first device
                let mut device = devices.pop().unwrap();
                assert_eq!(device.is_connected(), false);

                // Establish an ECP-2 connection with the device
                device.connect(load_ecp2_key()).await;
                assert_eq!(device.is_connected(), true);

                let info = device.get_info().await;
                println!("[-] Device: {:?}", info);
            }
            Err(_) => {}
        }
    }

    #[tokio::test]
    async fn toggle_device_power() {
        let mut found_device = false;
        for _retry in 0..5 {
            match &mut discover_devices(Duration::new(5, 0)).await {
                Ok(devices) => {
                    if devices.len() == 0 {
                        continue;
                    }

                    found_device = true;

                    // Work with the first device
                    let mut device = devices.pop().unwrap();
                    assert_eq!(device.is_connected(), false);

                    // Establish an ECP-2 connection with the device
                    device.connect(load_ecp2_key()).await;
                    assert_eq!(device.is_connected(), true);

                    let _ = device.toggle_power_state().await;

                    break;
                }
                Err(e) => assert!(false, "{:?}", e)
            }
        }

        assert!(found_device, "Failed to find devices")
    }
}

