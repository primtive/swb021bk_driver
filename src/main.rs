use crate::controller::{ButtonId, Controller};
use evdev::{Device, EventSummary, KeyCode};
use gtk::glib;
use std::sync::{OnceLock, mpsc};
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};

pub mod controller;
pub mod executor;

static ICON_IDLE: OnceLock<Icon> = OnceLock::new();
static ICON_LIBRESPRITE: OnceLock<Icon> = OnceLock::new();
static ICON_RNOTE: OnceLock<Icon> = OnceLock::new();
const DEVICE_NAME: &str = "SZ PING-IT INC.  [T605] Driver Inside Tablet Keyboard";

fn load_icon(path: &[u8]) -> Icon {
    let img = image::load_from_memory(path).unwrap().into_rgba8();
    let (w, h) = img.dimensions();

    Icon::from_rgba(img.into_raw(), w, h).unwrap()
}
fn init_icons() {
    ICON_IDLE.get_or_init(|| load_icon(include_bytes!("../assets/idle.png")));
    ICON_LIBRESPRITE.get_or_init(|| load_icon(include_bytes!("../assets/libresprite.png")));
    ICON_RNOTE.get_or_init(|| load_icon(include_bytes!("../assets/rnote.png")));
}
enum DriverMsg {
    SetPreset(i32),
    Enable,
    Disable,
}
enum TrayIcon {
    Idle,
    LibreSprite,
    Rnote,
}
enum DriverEvent {
    Disconnected,
    SetIcon(TrayIcon),
}

fn main() {
    init_icons();
    gtk::init().unwrap();
    let menu = Menu::new();
    let enable = MenuItem::new("Enable", true, None);
    let disable = MenuItem::new("Disable", true, None);
    let quit = MenuItem::new("Quit", true, None);

    let enable_id = enable.id().clone();
    let disable_id = disable.id().clone();

    menu.append(&enable).unwrap();
    menu.append(&disable).unwrap();
    menu.append(&quit).unwrap();
    let receiver = MenuEvent::receiver();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Tablet Remapper")
        .with_icon(ICON_IDLE.get().unwrap().clone())
        .build()
        .unwrap();
    let (driver_tx, driver_rx) = mpsc::channel::<DriverMsg>();
    let (event_tx, event_rx) = mpsc::channel::<DriverEvent>();
    std::thread::spawn(move || {
        if let Some((path, _)) =
            evdev::enumerate().find(|(_, device)| device.name() == Some(DEVICE_NAME))
        {
            let mut device = Device::open(path).unwrap();
            device.grab().unwrap();
            let mut ctr = Controller::new();
            println!("Device grabbed");

            std::thread::spawn(move || {
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
            });
            loop {
                while let Ok(msg) = driver_rx.recv() {
                    match msg {
                        DriverMsg::Enable => {
                            println!("enabled");
                            event_tx
                                .send(DriverEvent::SetIcon(TrayIcon::LibreSprite))
                                .unwrap();
                        }
                        DriverMsg::Disable => {
                            println!("disabled");
                        }
                        DriverMsg::SetPreset(preset) => {
                            println!("preset={preset}");
                        }
                    }
                }
            }
        } else {
            println!("could not find device");
            std::process::exit(1)
        }
    });
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        while let Ok(event) = receiver.try_recv() {
            if event.id == enable_id {
                driver_tx.send(DriverMsg::Enable).ok();
            }
            if event.id == disable_id {
                driver_tx.send(DriverMsg::Disable).ok();
            }
        }
        while let Ok(event) = event_rx.try_recv() {
            match event {
                DriverEvent::SetIcon(icon) => tray
                    .set_icon(match icon {
                        TrayIcon::Idle => Some(ICON_IDLE.get().unwrap().clone()),
                        TrayIcon::LibreSprite => Some(ICON_LIBRESPRITE.get().unwrap().clone()),
                        TrayIcon::Rnote => Some(ICON_RNOTE.get().unwrap().clone()),
                    })
                    .unwrap(),
                DriverEvent::Disconnected => {}
            }
        }

        glib::ControlFlow::Continue
    });
    glib::MainLoop::new(None, false).run();
}
