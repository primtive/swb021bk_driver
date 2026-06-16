use evdev::{AttributeSet, InputEvent, KeyCode, uinput::VirtualDevice};

pub struct Executor {
    dev: VirtualDevice,
    preset: Preset,
}

pub enum Preset {
    Rnote,
    LibreSprite,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Mode {
    #[default]
    Brush,
    Eraser,
    Select,
}
#[derive(Clone, Copy)]
pub enum Action {
    Delete,
    Undo,
    Redo,
    Pan(bool),
    ZoomIn,
    ZoomOut,
    SwitchMonitor,
    SetMode(Mode),
}
macro_rules! hotkey {
    ($mod:expr; $($key:expr),+ ) => {{
        [
            InputEvent::new(1, $mod.code(), 1),
            $(
                InputEvent::new(1, $key.code(), 1),
                InputEvent::new(1, $key.code(), 0),
            )+
            InputEvent::new(1, $mod.code(), 0),
        ]
    }};
    ($key:expr) => {{
        [
            InputEvent::new(1, $key.code(), 1),
            InputEvent::new(1, $key.code(), 0),
        ]
    }};
}
impl Executor {
    pub fn new() -> Self {
        let keys = AttributeSet::from_iter([
            KeyCode::KEY_1,
            KeyCode::KEY_4,
            KeyCode::KEY_5,
            KeyCode::KEY_Y,
            KeyCode::KEY_Z,
            KeyCode::KEY_KPMINUS,
            KeyCode::KEY_KPPLUS,
            KeyCode::KEY_DELETE,
            KeyCode::KEY_LEFTALT,
            KeyCode::KEY_LEFTCTRL,
            KeyCode::KEY_LEFTSHIFT,
            KeyCode::KEY_B,
            KeyCode::KEY_M,
            KeyCode::KEY_E,
        ]);
        let dev = VirtualDevice::builder()
            .unwrap()
            .name("SW021BK tablet driver")
            .with_keys(&keys)
            .unwrap()
            .build()
            .unwrap();
        Self {
            dev,
            preset: Preset::Rnote,
        }
    }
    pub fn set_preset(&mut self, preset: Preset) {
        self.preset = preset;
    }
    pub fn execute(&mut self, action: Action) {
        match self.preset {
            Preset::LibreSprite => match action {
                Action::SetMode(mode) => match mode {
                    Mode::Brush => {
                        self.dev.emit(&hotkey!(KeyCode::KEY_B)).unwrap();
                    }
                    Mode::Eraser => {
                        self.dev.emit(&hotkey!(KeyCode::KEY_E)).unwrap();
                    }
                    Mode::Select => {
                        self.dev.emit(&hotkey!(KeyCode::KEY_M)).unwrap();
                    }
                },
                Action::Delete => {
                    self.dev.emit(&hotkey!(KeyCode::KEY_DELETE)).unwrap();
                }
                Action::Pan(pan) => {
                    // eyedropper
                    let events = [InputEvent::new(
                        1,
                        KeyCode::KEY_LEFTALT.code(),
                        if pan { 1 } else { 0 },
                    )];
                    self.dev.emit(&events).unwrap();
                }
                Action::Redo => {
                    self.dev
                        .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_Y))
                        .unwrap();
                }
                Action::Undo => {
                    self.dev
                        .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_Z))
                        .unwrap();
                }
                Action::SwitchMonitor => {
                    todo!()
                }
                Action::ZoomIn => {
                    self.dev
                        .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_KPPLUS))
                        .unwrap();
                }
                Action::ZoomOut => {
                    self.dev
                        .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_KPMINUS))
                        .unwrap();
                }
            },
            Preset::Rnote => match action {
                Action::SetMode(mode) => match mode {
                    Mode::Brush => {
                        self.dev
                            .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_1))
                            .unwrap();
                    }
                    Mode::Eraser => {
                        self.dev
                            .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_4))
                            .unwrap();
                    }
                    Mode::Select => {
                        self.dev
                            .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_5))
                            .unwrap();
                    }
                },
                Action::Delete => {
                    self.dev.emit(&hotkey!(KeyCode::KEY_DELETE)).unwrap();
                }
                Action::Pan(pan) => {
                    let events = [InputEvent::new(
                        1,
                        KeyCode::KEY_LEFTALT.code(),
                        if pan { 1 } else { 0 },
                    )];
                    self.dev.emit(&events).unwrap();
                }
                Action::Redo => {
                    let events = [
                        InputEvent::new(1, KeyCode::KEY_LEFTCTRL.code(), 1),
                        InputEvent::new(1, KeyCode::KEY_LEFTSHIFT.code(), 1),
                        InputEvent::new(1, KeyCode::KEY_Z.code(), 1),
                        InputEvent::new(1, KeyCode::KEY_Z.code(), 0),
                        InputEvent::new(1, KeyCode::KEY_LEFTSHIFT.code(), 0),
                        InputEvent::new(1, KeyCode::KEY_LEFTCTRL.code(), 0),
                    ];
                    self.dev.emit(&events).unwrap();
                }
                Action::Undo => {
                    self.dev
                        .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_Z))
                        .unwrap();
                }
                Action::SwitchMonitor => {
                    todo!()
                }
                Action::ZoomIn => {
                    self.dev
                        .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_KPPLUS))
                        .unwrap();
                }
                Action::ZoomOut => {
                    self.dev
                        .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_KPMINUS))
                        .unwrap();
                }
            },
        }
    }
}
