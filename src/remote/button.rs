/// All known remote buttons                                                                <br/>
/// Legend:                                                                                 <br/>
///     * Optional - Requires device support                                                <br/>
///     * Android - Not officially documented, but found in Android source                  <br/>
///     * Undocumented - Not documented at all, but appears to work on my devices (^.^')    <br/>
#[allow(dead_code)]
pub enum Button {
    Home,
    Rewind,
    Forward,
    PlayPause,
    Select,
    Left,
    Right,
    Down,
    Up,
    Back,
    InstantReplay,
    Keyboard,
    Info,
    Backspace,
    Search,
    Enter,
    Stop,
    FindRemote,     // Optional
    VolumeDown,     // Optional
    VolumeMute,     // Optional
    VolumeUp,       // Optional
    Power,          // Android
    PowerOff,       // Optional
    PowerOn,        // Undocumented
    ChannelUp,      // Optional
    ChannelDown,    // Optional
    InputTuner,     // Optional
    InputHdmi1,     // Optional
    InputHdmi2,     // Optional
    InputHdmi3,     // Optional
    InputHdmi4,     // Optional
    InputAv1,       // Optional
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Numpad0,
    NumpadDot,      // Android
    NumpadRed,      // Android
    NumpadGreen,    // Android
    NumpadYellow,   // Android
    NumpadBlue,     // Android
    NumpadExit,     // Android
    Guide,          // Android
    TextSearch,     // Android
    VoiceSearch,    // Android
    Literal { character: char, },
}

impl ToString for Button {
    fn to_string(&self) -> String {
        match self {
            Button::Home => { String::from("Home") }
            Button::Rewind => { String::from("Rev") }
            Button::Forward => { String::from("Fwd") }
            Button::PlayPause => { String::from("Play") }
            Button::Select => { String::from("Select") }
            Button::Left => { String::from("Left") }
            Button::Right => { String::from("Right") }
            Button::Down => { String::from("Down") }
            Button::Up => { String::from("Up") }
            Button::Back => { String::from("Back") }
            Button::InstantReplay => { String::from("InstantReplay") }
            Button::Keyboard => { String::from("Keyboard") }
            Button::Info => { String::from("Info") }
            Button::Backspace => { String::from("Backspace") }
            Button::Search => { String::from("Search") }
            Button::Enter => { String::from("Enter") }
            Button::Stop => { String::from("Stop") }
            Button::FindRemote => { String::from("FindRemote") }
            Button::VolumeDown => { String::from("VolumeDown") }
            Button::VolumeMute => { String::from("VolumeMute") }
            Button::VolumeUp => { String::from("VolumeUp") }
            Button::Power => { String::from("Power") }
            Button::PowerOff => { String::from("PowerOff") }
            Button::PowerOn => { String::from("PowerOn") }
            Button::ChannelUp => { String::from("ChannelUp") }
            Button::ChannelDown => { String::from("ChannelDown") }
            Button::InputTuner => { String::from("InputTuner") }
            Button::InputHdmi1 => { String::from("InputHDMI1") }
            Button::InputHdmi2 => { String::from("InputHDMI2") }
            Button::InputHdmi3 => { String::from("InputHDMI3") }
            Button::InputHdmi4 => { String::from("InputHDMI4") }
            Button::InputAv1 => { String::from("InputAV1") }
            Button::Numpad1 => { String::from("1") }
            Button::Numpad2 => { String::from("2") }
            Button::Numpad3 => { String::from("3") }
            Button::Numpad4 => { String::from("4") }
            Button::Numpad5 => { String::from("5") }
            Button::Numpad6 => { String::from("6") }
            Button::Numpad7 => { String::from("7") }
            Button::Numpad8 => { String::from("8") }
            Button::Numpad9 => { String::from("9") }
            Button::Numpad0 => { String::from("0") }
            Button::NumpadDot => { String::from(".") }
            Button::NumpadRed => { String::from("Red") }
            Button::NumpadGreen => { String::from("Green") }
            Button::NumpadYellow => { String::from("Yellow") }
            Button::NumpadBlue => { String::from("Blue") }
            Button::NumpadExit => { String::from("Exit") }
            Button::Guide => { String::from("Guide") }
            Button::TextSearch => { String::from("TextSearch") }
            Button::VoiceSearch => { String::from("VoiceSearch") }
            Button::Literal { character } => { format!("Lit_-{}", character) }
        }
    }
}

impl From<&str> for Button {
    fn from(s: &str) -> Self {
        match s {
            "Home" => Button::Home,
            "Rev" => Button::Rewind,
            "Fwd" => Button::Forward,
            "Play" => Button::PlayPause,
            "Select" => Button::Select,
            "Left" => Button::Left,
            "Right" => Button::Right,
            "Down" => Button::Down,
            "Up" => Button::Up,
            "Back" => Button::Back,
            "InstantReplay" => Button::InstantReplay,
            "Keyboard" => Button::Keyboard,
            "Info" => Button::Info,
            "Backspace" => Button::Backspace,
            "Search" => Button::Search,
            "Enter" => Button::Enter,
            "Stop" => Button::Stop,
            "FindRemote" => Button::FindRemote,
            "VolumeDown" => Button::VolumeDown,
            "VolumeMute" => Button::VolumeMute,
            "VolumeUp" => Button::VolumeUp,
            "Power" => Button::Power,
            "PowerOff" => Button::PowerOff,
            "PowerOn" => Button::PowerOn,
            "ChannelUp" => Button::ChannelUp,
            "ChannelDown" => Button::ChannelDown,
            "InputTuner" => Button::InputTuner,
            "InputHDMI1" => Button::InputHdmi1,
            "InputHDMI2" => Button::InputHdmi2,
            "InputHDMI3" => Button::InputHdmi3,
            "InputHDMI4" => Button::InputHdmi4,
            "InputAV1" => Button::InputAv1,
            "1" => Button::Numpad1,
            "2" => Button::Numpad2,
            "3" => Button::Numpad3,
            "4" => Button::Numpad4,
            "5" => Button::Numpad5,
            "6" => Button::Numpad6,
            "7" => Button::Numpad7,
            "8" => Button::Numpad8,
            "9" => Button::Numpad9,
            "0" => Button::Numpad0,
            "." => Button::NumpadDot,
            "Red" => Button::NumpadRed,
            "Green" => Button::NumpadGreen,
            "Yellow" => Button::NumpadYellow,
            "Blue" => Button::NumpadBlue,
            "Exit" => Button::NumpadExit,
            "Guide" => Button::Guide,
            "TextSearch" => Button::TextSearch,
            "VoiceSearch" => Button::VoiceSearch,
            _ => panic!("Unrecognized button string")
        }
    }
}

impl From<String> for Button {
    fn from(s: String) -> Self {
        Button::from(s.as_str())
    }
}

impl From<char> for Button {
    fn from(c: char) -> Self {
        Button::Literal { character: urlencoding::encode(&format!("{}", c)).chars().next().unwrap() }
    }
}