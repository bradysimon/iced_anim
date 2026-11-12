use iced::{
    color,
    widget::{button, column, container, text, Row},
    Border, Color, Font, Length, Widget,
};
use iced_anim::animation_builder::AnimationBuilder;

#[derive(Debug, Clone)]
enum Message {
    ChangeColor(Color),
}

struct State {
    color: Color,
    text_color: Color,
}

impl Default for State {
    fn default() -> Self {
        Self {
            color: Self::COLORS[0],
            text_color: Self::text_color_for(Self::COLORS[0]),
        }
    }
}

impl State {
    /// The colors of the rainbow.
    const COLORS: [Color; 8] = [
        color!(0xFF, 0x00, 0x00), // Red
        color!(0xFF, 0xA5, 0x00), // Orange
        color!(0xFF, 0xFF, 0x00), // Yellow
        color!(0x00, 0xFF, 0x00), // Green
        color!(0x00, 0x00, 0xFF), // Blue
        color!(0x4B, 0x00, 0x82), // Indigo
        color!(0xEE, 0x82, 0xEE), // Violet
        Color::BLACK,
    ];

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeColor(color) => {
                self.color = color;
                self.text_color = Self::text_color_for(color);
            }
        }
    }

    fn text_color_for(color: Color) -> Color {
        if luminance_of(color) > 0.5 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    fn view(&self) -> impl Widget<Message> {
        let buttons = Row::with_children(Self::COLORS.iter().cloned().map(|color| {
            button(text(""))
                .style(move |_, _| iced::widget::button::Style {
                    background: Some(color.into()),
                    border: Border::default().rounded(6),
                    ..Default::default()
                })
                .on_press(Message::ChangeColor(color))
                .width(32)
                .height(32)
        }))
        .spacing(8);

        // You can animate multiple values at a time by passing a tuple to `AnimationBuilder`.
        let current_color =
            AnimationBuilder::new((self.color, self.text_color), |(color, text_color)| {
                container(
                    text(format!(
                        "0x{:02X}{:02X}{:02X}",
                        (255.0 * color.r) as u8,
                        (255.0 * color.g) as u8,
                        (255.0 * color.b) as u8
                    ))
                    .font(Font::MONOSPACE)
                    .color(text_color),
                )
                .style(move |_| iced::widget::container::Style {
                    background: Some(color.into()),
                    border: Border::default().rounded(6),
                    ..Default::default()
                })
                .center_x(Length::Fill)
                .center_y(Length::Fixed(30.0))
            })
            .animates_layout(true);

        column![buttons, current_color]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("Animated Color")
        .run()
}

/// Helper function to estimate the luminance of a color so we can adjust the text color.
fn luminance_of(color: Color) -> f32 {
    (0.299 * color.r + 0.587 * color.g + 0.114 * color.b) * color.a
}
