mod button;

/// Emulate use of a remote control, and help locate one
pub use crate::remote::button::Button;
use crate::Device;
use ecp::Set;

/// Adds additional remote-control
// NOTE: These require the device to be powered on
impl Device {
    /// Press a button on the remote
    // IMPLEMENTATION NOTE: If implementing a remote UI, it's best to use Device.set_power_state(TOGGLE) instead of sending PowerOn/PowerOff button presses
    pub async fn press_button(&mut self, button: Button) -> Result<(), String> {
        if !self.is_connected() {
            return Err(String::from("Not connected to device"));
        }

        let request = Set::PressKey { key: button.to_string() };
        match self.send_request(request.into()).await {
            None => Err(String::from("No response received for button press")),
            Some(_) => Ok(())
        }
    }

    /// Send multiple button presses back-to-back
    pub async fn press_buttons(&mut self, buttons: Vec<Button>) -> Result<(), String> {
        // Send buttons until we reach the end, stop if one doesn't send
        for b in buttons.into_iter() {
            if let Err(e) = self.press_button(b).await {
                return Err(e)
            }
        }
        Ok(())
    }

    // Convenience helper to send the "FindRemote" button press
    // NOTE: None of my devices have this, so I'm not sure how to test it...
    pub async fn find_remote(&mut self) -> Result<(), String> { self.press_button(Button::FindRemote).await }
}