use crate::executor::{
    Action::{self},
    Executor, Mode, Preset,
};

#[derive(Debug)]
pub enum ButtonId {
    Btn1,
    Btn2,
    Btn3,
    Btn4,
    Btn5,
    Btn6,
    PenPlus,
    PenMinus,
}

#[derive(Default)]
pub struct Button {
    pressed: bool,
    holded: bool,
}
#[derive(PartialEq, Debug)]
pub enum ButtonEvent {
    Click,
    HoldStart,
    HoldEnd,
}
impl Button {
    pub fn state(&mut self, status: i32) -> Option<ButtonEvent> {
        match status {
            0 => {
                let t = self.holded;
                self.pressed = false;
                self.holded = false;
                if t { Some(ButtonEvent::HoldEnd) } else { None }
            }
            1 => {
                self.pressed = true;
                self.holded = false;
                Some(ButtonEvent::Click)
            }
            2 => {
                let t = !self.holded;
                self.pressed = true;
                self.holded = true;
                if t {
                    Some(ButtonEvent::HoldStart)
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }
}

pub struct Controller {
    mode: Mode,
    btn_1: Button,
    btn_2: Button,
    btn_3: Button,
    btn_4: Button,
    btn_5: Button,
    btn_6: Button,
    pen_plus: Button,
    pen_minus: Button,
    pub executor: Executor,
    pan: bool,
    pan_holded: bool,
    krita_eraser_enabled: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            mode: Mode::default(),
            btn_1: Button::default(),
            btn_2: Button::default(),
            btn_3: Button::default(),
            btn_4: Button::default(),
            btn_5: Button::default(),
            btn_6: Button::default(),
            pen_plus: Button::default(),
            pen_minus: Button::default(),
            executor: Executor::new(),
            pan: false,
            pan_holded: false,
            krita_eraser_enabled: false,
        }
    }
    pub fn set_mode(&mut self, mode: Mode) {
        if mode != self.mode {
            self.mode = mode;
            if self.executor.preset == Preset::Krita {
                match (mode, self.krita_eraser_enabled) {
                    (Mode::Brush, true) | (Mode::Eraser, false) => {
                        self.executor.execute(Action::KritaToggleEraser);
                        self.krita_eraser_enabled = !self.krita_eraser_enabled;
                        return;
                    }
                    _ => {}
                }
            }
            self.executor.execute(Action::SetMode(mode));
        }
    }
    pub fn handle_btn_event(&mut self, btn_id: ButtonId, state: i32) {
        let btn_event: Option<ButtonEvent> = match btn_id {
            ButtonId::Btn1 => self.btn_1.state(state),
            ButtonId::Btn2 => self.btn_2.state(state),
            ButtonId::Btn3 => self.btn_3.state(state),
            ButtonId::Btn4 => self.btn_4.state(state),
            ButtonId::Btn5 => self.btn_5.state(state),
            ButtonId::Btn6 => self.btn_6.state(state),
            ButtonId::PenPlus => self.pen_plus.state(state),
            ButtonId::PenMinus => self.pen_minus.state(state),
        };
        if let Some(btn_event) = btn_event {
            println!("{:?} {:?}", &btn_id, &btn_event);
            match (btn_id, btn_event) {
                (ButtonId::Btn1, ButtonEvent::Click) => {
                    self.executor.execute(Action::Undo);
                }
                (ButtonId::Btn2, ButtonEvent::Click) => {
                    self.executor.execute(Action::Redo);
                }
                (ButtonId::Btn3, ButtonEvent::Click) => {
                    self.pan = !self.pan;
                    self.executor.execute(Action::Pan(self.pan));
                }
                (ButtonId::Btn3, ButtonEvent::HoldStart) => {
                    if self.pan {
                        self.pan_holded = true;
                    }
                }
                (ButtonId::Btn3, ButtonEvent::HoldEnd) => {
                    if self.pan_holded {
                        self.pan_holded = false;
                        self.pan = false;
                        self.executor.execute(Action::Pan(self.pan));
                    }
                }
                (ButtonId::Btn4, ButtonEvent::Click) => match self.mode {
                    Mode::Brush => self.set_mode(Mode::Eraser),
                    Mode::Eraser => self.set_mode(Mode::Brush),
                    Mode::Select => self.executor.execute(Action::Delete),
                },
                (ButtonId::Btn5, ButtonEvent::Click) => {
                    self.executor.execute(Action::ZoomIn);
                }
                (ButtonId::Btn6, ButtonEvent::Click) => {
                    self.executor.execute(Action::ZoomOut);
                }
                (ButtonId::PenPlus, ButtonEvent::Click) => match self.mode {
                    Mode::Brush => self.set_mode(Mode::Eraser),
                    Mode::Eraser => self.set_mode(Mode::Brush),
                    Mode::Select => self.executor.execute(Action::Delete),
                },
                (ButtonId::PenMinus, ButtonEvent::Click) => {
                    if self.executor.preset == Preset::Krita {
                        self.executor.execute(Action::Custom1);
                    } else {
                        self.set_mode(if self.mode != Mode::Select {
                            Mode::Select
                        } else {
                            Mode::Brush
                        });
                    }
                }
                _ => {}
            }
            println!("{}", self.pan);
            println!("{}", self.pan_holded);
        }
    }
}
