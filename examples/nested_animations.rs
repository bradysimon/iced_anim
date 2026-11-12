//! Note: this example doesn't yet fully work as expected. The core issue is needing to rebuild
//! the `cached_element` in `AnimationBuilder::on_event`, but trying to propagate events to the
//! updated widget causes a panic in the `preview_motion` example. This is related to the
//! `push_maybe` calls in that example, although I haven't figured out a workaround yet.
use iced::{
    widget::{button, column, container, row, text},
    Border, Color, Length, Widget,
};
use iced_anim::{transition::Easing, Animated, Animation, Event};

#[derive(Debug, Clone)]
enum Message {
    AdjustAll,
    ChangeSize(Event<f32>),
    ChangeColor(Event<Color>),
}

struct State {
    size: Animated<f32>,
    color: Animated<Color>,
}

const CYAN: Color = Color::from_rgb(0.0, 0.8, 0.8);
const MAGENTA: Color = Color::from_rgb(1.0, 0.0, 1.0);

impl Default for State {
    fn default() -> Self {
        Self {
            size: Animated::transition(50.0, Easing::EASE),
            color: Animated::transition(CYAN, Easing::EASE),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::AdjustAll => {
                self.size.update(
                    if *self.size.target() == 50.0 {
                        150.0
                    } else {
                        50.0
                    }
                    .into(),
                );
                self.color.update(
                    if *self.color.target() == CYAN {
                        MAGENTA
                    } else {
                        CYAN
                    }
                    .into(),
                )
            }
            Message::ChangeSize(event) => self.size.update(event),
            Message::ChangeColor(event) => self.color.update(event),
        }
    }

    fn view(&self) -> impl Widget<Message> + '_ {
        let buttons = row![
            button(text("Adjust size")).on_press(Message::ChangeSize(
                if *self.size.target() == 50.0 {
                    150.0
                } else {
                    50.0
                }
                .into()
            )),
            button(text("Adjust color")).on_press(Message::ChangeColor(
                if *self.color.target() == CYAN {
                    MAGENTA
                } else {
                    CYAN
                }
                .into()
            )),
            button(text("Adjust all")).on_press(Message::AdjustAll),
        ]
        .spacing(8);

        let animated_box = Animation::new(
            &self.size,
            Animation::new(
                &self.color,
                container(text((*self.size.value() as isize).to_string()))
                    .style(move |_: &iced::Theme| container::Style {
                        border: Border {
                            color: *self.color.value(),
                            width: 1.0,
                            radius: 6.0.into(),
                        },
                        background: Some((*self.color.value()).into()),
                        ..Default::default()
                    })
                    .center(*self.size.value()),
            )
            .on_update(Message::ChangeColor),
        )
        .on_update(Message::ChangeSize);

        column![buttons, animated_box]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("Nested AnimationBuilders")
        .run()
}
