use crate::{
    controller::{ButtonId, Controller},
    executor::Preset,
};
use evdev::{Device, EventSummary, KeyCode};
use gtk::glib;
use std::sync::{OnceLock, mpsc};
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};

pub mod controller;
pub mod executor;

static ICON_IDLE: OnceLock<Icon> = OnceLock::new();
static ICON_LIBRESPRITE: OnceLock<Icon> = OnceLock::new();
static ICON_RNOTE: OnceLock<Icon> = OnceLock::new();
static ICON_KRITA: OnceLock<Icon> = OnceLock::new();
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
    ICON_KRITA.get_or_init(|| load_icon(include_bytes!("../assets/krita.png")));
}
enum TrayIcon {
    Idle,
    Preset(Preset),
}
enum DriverMsg {
    SetPreset(Preset),
    Enable,
    Disable,
}
enum DriverEvent {
    SetIcon(TrayIcon),
}

fn main() {
    init_icons();
    gtk::init().unwrap();
    let menu = Menu::new();

    let rnote = MenuItem::new("Rnote", true, None);
    let libresprite = MenuItem::new("LibreSprite", true, None);
    let krita = MenuItem::new("Krita", true, None);
    let enable = MenuItem::new("Enable", true, None);
    let disable = MenuItem::new("Disable", true, None);
    let quit = MenuItem::new("Quit", true, None);

    let rnote_id = rnote.id().clone();
    let libresprite_id = libresprite.id().clone();
    let krita_id = krita.id().clone();
    let enable_id = enable.id().clone();
    let disable_id = disable.id().clone();
    let quit_id = quit.id().clone();

    menu.append_items(&[
        &rnote,
        &libresprite,
        &krita,
        &PredefinedMenuItem::separator(),
        &enable,
        &disable,
        &PredefinedMenuItem::separator(),
        &quit,
    ])
    .unwrap();
    let receiver = MenuEvent::receiver();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(ICON_IDLE.get().unwrap().clone())
        .build()
        .unwrap();
    let (driver_tx, driver_rx) = mpsc::channel::<DriverMsg>();
    let (event_tx, event_rx) = mpsc::channel::<DriverEvent>();
    std::thread::spawn(move || {
        let mut device: Option<Device> = None;
        let mut ctr = Controller::new();
        loop {
            if let Some(device) = device.as_mut() {
                match device.fetch_events() {
                    Ok(events) => {
                        for event in events {
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
                    Err(_) => {
                        // println!("{err}");
                        // println!("disconnetcted");
                        // event_tx.send(DriverEvent::SetIcon(TrayIcon::Idle)).unwrap();
                    }
                }
            }
            while let Ok(msg) = driver_rx.try_recv() {
                match msg {
                    DriverMsg::Enable => {
                        if let Some((path, _)) = evdev::enumerate()
                            .find(|(_, device)| device.name() == Some(DEVICE_NAME))
                        {
                            println!("{path:?}");
                            let mut dev = Device::open(path).unwrap();
                            dev.grab().unwrap();
                            dev.set_nonblocking(true).unwrap();
                            device = Some(dev);
                            println!("Device grabbed");
                            event_tx
                                .send(DriverEvent::SetIcon(TrayIcon::Preset(ctr.executor.preset)))
                                .unwrap();
                        } else {
                            println!("could not find the device")
                        }
                    }
                    DriverMsg::Disable => {
                        println!("disabled");
                        device = None;
                        event_tx.send(DriverEvent::SetIcon(TrayIcon::Idle)).unwrap();
                    }
                    DriverMsg::SetPreset(preset) => {
                        ctr.executor.set_preset(preset);
                        println!("preset changed");
                        if device.is_some() {
                            event_tx
                                .send(DriverEvent::SetIcon(TrayIcon::Preset(ctr.executor.preset)))
                                .unwrap();
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        while let Ok(event) = receiver.try_recv() {
            if event.id == rnote_id {
                driver_tx.send(DriverMsg::SetPreset(Preset::Rnote)).ok();
            } else if event.id == libresprite_id {
                driver_tx
                    .send(DriverMsg::SetPreset(Preset::LibreSprite))
                    .ok();
            } else if event.id == krita_id {
                driver_tx.send(DriverMsg::SetPreset(Preset::Krita)).ok();
            } else if event.id == enable_id {
                driver_tx.send(DriverMsg::Enable).ok();
            } else if event.id == disable_id {
                driver_tx.send(DriverMsg::Disable).ok();
            } else if event.id == quit_id {
                std::process::exit(0);
            }
        }
        while let Ok(event) = event_rx.try_recv() {
            match event {
                DriverEvent::SetIcon(icon) => tray
                    .set_icon(match icon {
                        TrayIcon::Idle => Some(ICON_IDLE.get().unwrap().clone()),
                        TrayIcon::Preset(Preset::LibreSprite) => {
                            Some(ICON_LIBRESPRITE.get().unwrap().clone())
                        }
                        TrayIcon::Preset(Preset::Rnote) => Some(ICON_RNOTE.get().unwrap().clone()),
                        TrayIcon::Preset(Preset::Krita) => Some(ICON_KRITA.get().unwrap().clone()),
                    })
                    .unwrap(),
            }
        }

        glib::ControlFlow::Continue
    });
    glib::MainLoop::new(None, false).run();
}
