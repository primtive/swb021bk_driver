use evdev::{AttributeSet, InputEvent, KeyCode, uinput::VirtualDevice};

pub struct Executor {
    dev: VirtualDevice,
    pub preset: Preset,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Preset {
    #[default]
    Rnote,
    LibreSprite,
    Krita,
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
    Custom1,
    ZoomIn,
    ZoomOut,
    SetMode(Mode),
    KritaToggleEraser,
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
            KeyCode::KEY_SPACE,
            KeyCode::KEY_KPMINUS,
            KeyCode::KEY_KPPLUS,
            KeyCode::KEY_DELETE,
            KeyCode::KEY_LEFTALT,
            KeyCode::KEY_LEFTCTRL,
            KeyCode::KEY_LEFTSHIFT,
            KeyCode::KEY_B,
            KeyCode::KEY_M,
            KeyCode::KEY_E,
            KeyCode::KEY_R,
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
            preset: Preset::default(),
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
                Action::Custom1 => {
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
                Action::KritaToggleEraser => {}
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
                Action::Custom1 => {
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
                Action::KritaToggleEraser => {}
            },
            Preset::Krita => match action {
                Action::SetMode(mode) => match mode {
                    Mode::Brush => {
                        self.dev.emit(&hotkey!(KeyCode::KEY_B)).unwrap();
                    }
                    Mode::Eraser => {
                        self.dev.emit(&hotkey!(KeyCode::KEY_E)).unwrap();
                    }
                    Mode::Select => {
                        self.dev
                            .emit(&hotkey!(KeyCode::KEY_LEFTCTRL; KeyCode::KEY_R))
                            .unwrap();
                    }
                },
                Action::Delete => {
                    self.dev.emit(&hotkey!(KeyCode::KEY_DELETE)).unwrap();
                }
                Action::Pan(pan) => {
                    // eyedropper
                    let events = [InputEvent::new(
                        1,
                        KeyCode::KEY_SPACE.code(),
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
                Action::Custom1 => {
                    let events = [
                        InputEvent::new(1, KeyCode::KEY_LEFTCTRL.code(), 1),
                        InputEvent::new(1, KeyCode::KEY_LEFTALT.code(), 1),
                        InputEvent::new(1, KeyCode::KEY_1.code(), 1),
                        InputEvent::new(1, KeyCode::KEY_1.code(), 0),
                        InputEvent::new(1, KeyCode::KEY_LEFTALT.code(), 0),
                        InputEvent::new(1, KeyCode::KEY_LEFTCTRL.code(), 0),
                    ];
                    self.dev.emit(&events).unwrap();
                }
                Action::ZoomIn => {
                    self.dev.emit(&hotkey!(KeyCode::KEY_KPPLUS)).unwrap();
                }
                Action::ZoomOut => {
                    self.dev.emit(&hotkey!(KeyCode::KEY_KPMINUS)).unwrap();
                }
                Action::KritaToggleEraser => {
                    self.dev.emit(&hotkey!(KeyCode::KEY_E)).unwrap();
                }
            },
        }
    }
}
