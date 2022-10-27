koru
====
_Roku client library written in Rust_

## Supported features
* Device discovery
* Remote controls via ecp library

## Objects

### Device

#### Properties
* `connection:  Option<Connection>`
* `ipv4:        String`
* `port:        i32`
* `name:        String`
* `network:     NetworkType`
* `mac_wol:     Option<[u8; 6]>`
* `mac_wlan:    Option<[u8; 6]>`
* `mac_eth:     Option<[u8; 6]>`
* `power_state: PowerState`

#### Methods
* `fn new(ipv4: &str, port: i32) -> Device`  
  Constructor
* `async fn connect(&mut self, key: Vec<u8>) -> bool`  
  Open an ECP connection and authenticate, returning auth result
* `async fn send_request(&mut self, request: Request) -> Option<Response>`  
  Send an ECP request to the device & return response
* `fn launch_app_by_id(app: &App) : Result<bool, String>`  
  Launches an app of specified id
* `async fn update_self(&mut self)`  
  Forces the device to fetch its most recent info

#### Accessors
* `fn is_connected(&self) -> bool`  
  Whether or not the device is connected
* `fn get_info() : Result<HashMap<String, String>, String>`  
  Return parsed device info
* `fn get_power_state() : POWERSTATE`  
  Get device power state
* `fn get_installed_apps() : Result<Vec<App>, String>`  
  Return a Vec of installed apps

### App

#### Properties
* `id: i32`
* `apptype: String`
* `version: String,`
* `name: String,`
* `icon: Option<Vec<u8>>`

#### Methods
* `fetch_icon()  :  Result<Vec<u8>, String>`  
  Fetches the icon from the device for this app