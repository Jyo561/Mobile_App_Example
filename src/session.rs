#[derive(Clone, Copy, PartialEq)]
pub enum SessionType {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

impl SessionType {
    pub fn duration(self) -> u64 {
        match self {
            Self::Pomodoro => 25 * 60,
            Self::ShortBreak => 5 * 60,
            Self::LongBreak => 15 * 60,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Pomodoro => "Pomodoro",
            Self::ShortBreak => "Short Break",
            Self::LongBreak => "Long Break",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Pomodoro => Self::ShortBreak,
            Self::ShortBreak => Self::Pomodoro,
            Self::LongBreak => Self::Pomodoro,
        }
    }
}
