use evdev::{Device, EventSummary, KeyCode};
use swb021bk_driver::{
    controller::{ButtonId, Controller},
    executor::Preset,
};

const DEVICE_NAME: &str = "SZ PING-IT INC.  [T605] Driver Inside Tablet Keyboard";

fn main() {
    if let Some((path, _)) =
        evdev::enumerate().find(|(_, device)| device.name() == Some(DEVICE_NAME))
    {
        let mut device = Device::open(path).unwrap();
        device.grab().unwrap();
        let mut ctr = Controller::new();
        println!("Device grabbed");
        if let Some(preset) = std::env::args().nth(1) {
            match preset.as_str() {
                "-l" => {
                    ctr.executor.set_preset(Preset::LibreSprite);
                    println!("using libresprite preset");
                }
                "-k" => {
                    ctr.executor.set_preset(Preset::Krita);
                    println!("using krita preset");
                }
                _ => {
                    println!("using rnote (default) preset");
                }
            }
        }

        loop {
            for event in device.fetch_events().unwrap() {
                if let EventSummary::Key(_, keycode, state) = event.destructure() {
                    let btn_id: Option<ButtonId> = match keycode {
                        KeyCode::KEY_KPMINUS => Some(ButtonId::Btn1),
                        KeyCode::KEY_KPPLUS => Some(ButtonId::Btn2),
                        KeyCode::KEY_LEFTBRACE => Some(ButtonId::Btn3),
                        KeyCode::KEY_RIGHTBRACE => Some(ButtonId::Btn4),
                        KeyCode::KEY_TAB => Some(ButtonId::Btn5),
                        KeyCode::KEY_SPACE => Some(ButtonId::Btn6),
                        KeyCode::KEY_Y => Some(ButtonId::PenPlus),
                        KeyCode::KEY_Z => Some(ButtonId::PenMinus),
                        KeyCode::KEY_LEFTCTRL | _ => None,
                    };
                    if let Some(btn_id) = btn_id {
                        ctr.handle_btn_event(btn_id, state);
                    }
                };
            }
        }
    } else {
        println!("could not find device");
        std::process::exit(1)
    }
}
