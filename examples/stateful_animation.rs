//! An example of a stateful animation using the `Animation` widget instead of
//! the `AnimationBuilder` widget. Stateful animations store the animation
//! within the app state whereas the `AnimationBuilder` widget stores the
//! animation within the widget tree.
use iced::{
    time::Duration,
    widget::{container, mouse_area, space},
    Border, Color,
    Length::{self, Fill},
    Point, Size, Subscription, Theme, Widget,
};
use iced_anim::{spring::Motion, Animated, Animation, Event};

#[derive(Debug, Clone)]
enum Message {
    UpdatePosition(Event<Point>),
    Resized(Size),
}

struct State {
    /// The current position of the bubble in the window.
    position: Animated<Point>,
    /// The size of the window to help generate a color.
    size: Size,
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: Animated::spring(
                Point::new(50.0, 50.0),
                Motion {
                    response: Duration::from_millis(500),
                    damping: 0.6,
                },
            ),
            size: Size::new(1024.0, 768.0),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::UpdatePosition(event) => self.position.update(event),
            Message::Resized(size) => self.size = size,
        }
    }

    /// Listens for window resize events to ensure the bubble color is accurate.
    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _, _| match event {
            iced::Event::Window(iced::window::Event::Resized(size)) => Some(Message::Resized(size)),
            _ => None,
        })
    }

    fn view(&self) -> impl Widget<Message> + '_ {
        let size = self.size;
        container(
            mouse_area(
                Animation::new(
                    &self.position,
                    container(
                        container(space().width(Length::Fill).height(Length::Fill))
                            .width(self.position.value().x)
                            .height(self.position.value().y)
                            .style(move |_: &Theme| container::Style {
                                border: Border::default().rounded(8),
                                background: Some(get_color(self.position.value(), size).into()),
                                ..Default::default()
                            }),
                    )
                    .width(Fill)
                    .height(Fill),
                )
                .on_update(Message::UpdatePosition),
            )
            .on_move(|point| Message::UpdatePosition(point.into())),
        )
        .width(Fill)
        .height(Fill)
    }
}

/// Returns a color based on the position and size of the screen.
fn get_color(position: &Point, size: Size) -> Color {
    let width = size.width.max(1.0);
    let height = size.height.max(1.0);
    let x_ratio = (position.x / width).clamp(0.0, 1.0);
    let y_ratio = (position.y / height).clamp(0.0, 1.0);

    Color::from_rgb(x_ratio, y_ratio, 1.0 - x_ratio)
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .subscription(State::subscription)
        .title("Stateful Animation")
        .run()
}
