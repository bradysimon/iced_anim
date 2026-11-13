use iced::{
    gradient::{ColorStop, Linear},
    time::Duration,
    widget::{checkbox, column, container, row, text},
    Alignment::Center,
    Background, Border, Color, Gradient,
    Length::Fill,
    Theme, Widget,
};
use iced_anim::{
    spring::Motion,
    transition::Easing,
    widget::button::{button, danger, primary, Status},
};
use std::{f32::consts::PI, sync::LazyLock};

#[derive(Debug, Clone)]
enum Message {
    Adjust(i32),
    DisableButtons(bool),
}

#[derive(Debug, Default)]
struct State {
    /// The counter value.
    counter: i32,
    /// Whether the buttons in this example are disabled
    is_disabled: bool,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Adjust(amount) => self.counter += amount,
            Message::DisableButtons(disabled) => self.is_disabled = disabled,
        };
    }

    fn view(&self) -> impl Widget<Message> + '_ {
        let is_enabled = !self.is_disabled;
        container(
            column![
                button(text("+"))
                    .on_press_maybe(is_enabled.then_some(Message::Adjust(1)))
                    .animation(Easing::EASE)
                    .style(primary),
                row![
                    button(text("-10"))
                        .on_press_maybe(is_enabled.then_some(Message::Adjust(-10)))
                        .animation(Motion::default())
                        .style(danger_gradient),
                    text(self.counter.to_string()).size(24).width(60.0).center(),
                    button(text("+10"))
                        .on_press_maybe(is_enabled.then_some(Message::Adjust(10)))
                        .animation(Motion::default())
                        .style(success_gradient),
                ]
                .spacing(8),
                button(text("-"))
                    .on_press_maybe(is_enabled.then_some(Message::Adjust(-1)))
                    .animation(Easing::EASE)
                    .style(danger),
                button(text("Reset").size(20))
                    .on_press_maybe(is_enabled.then_some(Message::Adjust(-self.counter)))
                    .animation(Motion {
                        response: Duration::from_millis(1000),
                        damping: Motion::SMOOTH.damping(),
                    })
                    .style(rainbow_style),
                checkbox(self.is_disabled)
                    .label("Disabled Buttons")
                    .on_toggle(Message::DisableButtons)
            ]
            .align_x(Center)
            .spacing(8)
            .padding(8),
        )
        .center(Fill)
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .theme(iced::Theme::CatppuccinFrappe)
        .title("Animated Widgets")
        .run()
}

/// A disabled gradient for the custom button styling.
static DISABLED_GRADIENT: LazyLock<Gradient> = LazyLock::new(|| {
    Gradient::Linear(Linear::new(2.0 * PI).add_stops([
        ColorStop {
            offset: 0.0,
            color: Color::from_rgb(0.7, 0.7, 0.8),
        },
        ColorStop {
            offset: 0.5,
            color: Color::from_rgb(0.5, 0.5, 0.6),
        },
        ColorStop {
            offset: 1.0,
            color: Color::from_rgb(0.3, 0.3, 0.4),
        },
    ]))
});

fn danger_gradient(theme: &Theme, status: Status) -> iced::widget::button::Style {
    let danger = theme.palette().danger;
    let gradient = match status {
        Status::Disabled => *DISABLED_GRADIENT,
        Status::Active => Gradient::Linear(Linear::new(0).add_stops([
            ColorStop {
                offset: 0.0,
                color: Color::from_rgb(0.6, 0.0, 0.0),
            },
            ColorStop {
                offset: 0.5,
                color: Color::from_rgb(0.8, 0.0, 0.0),
            },
            ColorStop {
                offset: 1.0,
                color: Color::from_rgb(1.0, 0.0, 0.0),
            },
        ])),
        _ => Gradient::Linear(Linear::new(0).add_stops([
            ColorStop {
                offset: 0.0,
                color: Color::from_rgb(1.0, 0.6, 0.2),
            },
            ColorStop {
                offset: 0.5,
                color: Color::from_rgb(1.0, 0.4, 0.1),
            },
            ColorStop {
                offset: 1.0,
                color: Color::from_rgb(0.8, 0.3, 0.0),
            },
        ])),
    };
    iced::widget::button::Style {
        background: Some(Background::Gradient(gradient)),
        text_color: danger.base.text,
        border: Border::default().rounded(24.0),
        ..Default::default()
    }
}

fn success_gradient(theme: &Theme, status: Status) -> iced::widget::button::Style {
    let success = theme.palette().success;
    let gradient = match status {
        Status::Disabled => *DISABLED_GRADIENT,
        Status::Active => Gradient::Linear(Linear::new(0).add_stops([
            ColorStop {
                offset: 0.0,
                color: Color::from_rgb(0.8, 0.9, 1.0),
            },
            ColorStop {
                offset: 0.5,
                color: Color::from_rgb(0.4, 0.6, 0.9),
            },
            ColorStop {
                offset: 1.0,
                color: Color::from_rgb(0.0, 0.3, 0.8),
            },
        ])),
        _ => Gradient::Linear(Linear::new(0).add_stops([
            ColorStop {
                offset: 0.0,
                color: Color::from_rgb(0.8, 1.0, 0.8),
            },
            ColorStop {
                offset: 0.5,
                color: Color::from_rgb(0.4, 0.9, 0.4),
            },
            ColorStop {
                offset: 1.0,
                color: Color::from_rgb(0.0, 0.8, 0.0),
            },
        ])),
    };
    iced::widget::button::Style {
        background: Some(Background::Gradient(gradient)),
        text_color: success.base.text,
        border: Border::default().rounded(24.0),
        ..Default::default()
    }
}

fn rainbow_style(_theme: &Theme, status: Status) -> iced::widget::button::Style {
    const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
    const ORANGE: Color = Color::from_rgb(1.0, 0.5, 0.0);
    const YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.0);
    const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0);
    const INDIGO: Color = Color::from_rgb(0.29, 0.0, 0.51);
    const VIOLET: Color = Color::from_rgb(0.56, 0.0, 1.0);

    let gradient = match status {
        Status::Disabled => *DISABLED_GRADIENT,
        Status::Active => Gradient::Linear(Linear::new(0).add_stops([
            ColorStop {
                offset: 0.0,
                color: RED,
            },
            ColorStop {
                offset: 0.5,
                color: ORANGE,
            },
            ColorStop {
                offset: 1.0,
                color: YELLOW,
            },
        ])),
        _ => Gradient::Linear(Linear::new(PI).add_stops([
            ColorStop {
                offset: 0.0,
                color: VIOLET,
            },
            ColorStop {
                offset: 0.5,
                color: INDIGO,
            },
            ColorStop {
                offset: 1.0,
                color: BLUE,
            },
        ])),
    };
    iced::widget::button::Style {
        text_color: match status {
            Status::Active => Color::BLACK,
            _ => Color::WHITE,
        },
        background: Some(Background::Gradient(gradient)),
        border: match status {
            Status::Active => Border::default().width(2.0).color(Color::BLACK).rounded(0),
            _ => Border::default().width(2.0).color(Color::WHITE).rounded(24),
        },
        ..Default::default()
    }
}
