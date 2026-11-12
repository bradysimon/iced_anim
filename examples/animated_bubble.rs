use iced::{
    time::Duration,
    widget::{container, space, MouseArea},
    Border, Color, Length, Padding, Point, Size, Subscription, Theme, Widget,
};
use iced_anim::{spring::Motion, AnimationBuilder};

const BUBBLE_SIZE: f32 = 50.0;

#[derive(Debug, Clone)]
enum Message {
    MoveTo(Point),
    Resized(Size),
}

struct State {
    /// The current position of the bubble in the window.
    position: Point,
    /// The size of the window to help generate a color.
    size: Size,
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: Point::new(BUBBLE_SIZE / 2.0, BUBBLE_SIZE / 2.0),
            size: Size::new(1024.0, 768.0),
        }
    }
}

/// Returns a color based on the position and size of the bubble.
fn get_bubble_color(position: Point, size: Size) -> Color {
    let width = size.width.max(1.0);
    let height = size.height.max(1.0);
    let x_ratio = (position.x / width).clamp(0.0, 1.0);
    let y_ratio = (position.y / height).clamp(0.0, 1.0);

    Color::from_rgb(x_ratio, y_ratio, 1.0 - x_ratio)
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::MoveTo(position) => self.position = position,
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
            AnimationBuilder::new(self.position, move |position| {
                MouseArea::new(
                    container(
                        container(space().width(Length::Fill).height(Length::Fill))
                            .width(Length::Fixed(BUBBLE_SIZE))
                            .height(Length::Fixed(BUBBLE_SIZE))
                            .style(move |_: &Theme| iced::widget::container::Style {
                                border: Border::default().rounded(BUBBLE_SIZE / 2.0),
                                background: Some(get_bubble_color(position, size).into()),
                                ..Default::default()
                            }),
                    )
                    .padding(
                        Padding::ZERO
                            .top(position.y - BUBBLE_SIZE / 2.0)
                            .left(position.x - BUBBLE_SIZE / 2.0),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
                .on_move(Message::MoveTo)
            })
            .animation(Motion {
                response: Duration::from_millis(500),
                damping: 0.6,
            })
            .animates_layout(true),
        )
        .width(Length::Fill)
        .height(Length::Fill)
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .subscription(State::subscription)
        .title("Animated Bubble")
        .run()
}
