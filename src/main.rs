use iced::{
    alignment,
    border::Radius,
    widget::{button, column, container, row, text},
    Background, Color, Element, Length, Shadow, Subscription, Task,
};
use std::time::Duration;

fn main() -> iced::Result {
    iced::application("Pomodoro", update, view)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Toggle,
    Reset,
}

struct Pomodoro {
    seconds: u32,
    running: bool,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            seconds: 25 * 60,
            running: false,
        }
    }
}

fn update(state: &mut Pomodoro, message: Message) -> Task<Message> {
    match message {
        Message::Tick => {
            if state.running && state.seconds > 0 {
                state.seconds -= 1;
            }

            if state.seconds == 0 {
                state.running = false;
            }
        }

        Message::Toggle => {
            state.running = !state.running;
        }

        Message::Reset => {
            state.running = false;
            state.seconds = 25 * 60;
        }
    }

    Task::none()
}

fn subscription(state: &Pomodoro) -> Subscription<Message> {
    if state.running {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    } else {
        Subscription::none()
    }
}

fn view(state: &Pomodoro) -> Element<'_, Message> {
    let minutes = state.seconds / 60;
    let seconds = state.seconds % 60;

    // --------------------------------------------------
    // Colors
    // --------------------------------------------------

    let background = Color::from_rgb(0.88, 0.89, 0.91);

    let dark_text = Color::from_rgb(0.25, 0.27, 0.30);

    let accent = Color::from_rgb(0.85, 0.30, 0.30);

    // --------------------------------------------------
    // Title
    // --------------------------------------------------

    let title = text("POMODORO")
        .size(24)
        .color(dark_text);

    let subtitle = text(if state.running {
        "FOCUS MODE"
    } else {
        "READY TO FOCUS"
    })
    .size(13)
    .color(Color::from_rgb(0.45, 0.46, 0.48));

    // --------------------------------------------------
    // Timer
    // --------------------------------------------------

    let timer = text(format!("{minutes:02}:{seconds:02}"))
        .size(72)
        .color(dark_text)
        .align_x(alignment::Horizontal::Center);

    let timer_card = container(timer)
        .width(Length::Fixed(300.0))
        .height(Length::Fixed(180.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(background)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(30.0),
            },
            shadow: Shadow {
                color: Color {
                    a: 0.25,
                    ..Color::BLACK
                },
                offset: iced::Vector::new(8.0, 8.0),
                blur_radius: 16.0,
            },
            ..Default::default()
        });

    // --------------------------------------------------
    // Start / Pause button
    // --------------------------------------------------

    let toggle_button = button(
        text(if state.running {
            "PAUSE"
        } else {
            "START"
        })
        .size(16)
        .color(Color::WHITE),
    )
    .width(Length::Fixed(140.0))
    .height(Length::Fixed(55.0))
    .style(move |_theme, status| {
        let shadow = match status {
            button::Status::Pressed => Shadow {
                color: Color {
                    a: 0.15,
                    ..Color::BLACK
                },
                offset: iced::Vector::new(2.0, 2.0),
                blur_radius: 5.0,
            },

            _ => Shadow {
                color: Color {
                    a: 0.25,
                    ..Color::BLACK
                },
                offset: iced::Vector::new(6.0, 6.0),
                blur_radius: 10.0,
            },
        };

        button::Style {
            background: Some(Background::Color(accent)),
            text_color: Color::WHITE,
            border: iced::Border {
                radius: Radius::from(28.0),
                ..Default::default()
            },
            shadow,
        }
    })
    .on_press(Message::Toggle);

    // --------------------------------------------------
    // Reset button
    // --------------------------------------------------

    let reset_button = button(
        text("RESET")
            .size(16)
            .color(dark_text),
    )
    .width(Length::Fixed(140.0))
    .height(Length::Fixed(55.0))
    .style(move |_theme, status| {
        let shadow = match status {
            button::Status::Pressed => Shadow {
                color: Color {
                    a: 0.12,
                    ..Color::BLACK
                },
                offset: iced::Vector::new(2.0, 2.0),
                blur_radius: 4.0,
            },

            _ => Shadow {
                color: Color {
                    a: 0.22,
                    ..Color::BLACK
                },
                offset: iced::Vector::new(6.0, 6.0),
                blur_radius: 10.0,
            },
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: dark_text,
            border: iced::Border {
                radius: Radius::from(28.0),
                ..Default::default()
            },
            shadow,
        }
    })
    .on_press(Message::Reset);

    // --------------------------------------------------
    // Buttons
    // --------------------------------------------------

    let buttons = row![
        toggle_button,
        reset_button
    ]
    .spacing(25)
    .align_y(iced::Alignment::Center);

    // --------------------------------------------------
    // Main card
    // --------------------------------------------------

    let content = column![
        title,
        subtitle,
        timer_card,
        buttons,
    ]
    .align_x(iced::Alignment::Center)
    .spacing(15);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(40)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(background)),
            ..Default::default()
        })
        .into()
}
