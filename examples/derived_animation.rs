use iced::{
    widget::{button, column, container, text},
    Border, Color, Length, Widget,
};
use iced_anim::AnimationBuilder;
use iced_anim::{transition::Easing, Animate};

/// A custom struct with all animatable properties that derives `Animate`
#[derive(Animate, Clone, Debug, PartialEq)]
struct CustomRect {
    width: f32,
    height: f32,
    border_radius: f32,
    color: Color,
}

impl CustomRect {
    /// A fake randomizer that will adjust the rectangle's properties
    fn randomize(&mut self) {
        self.border_radius = (self.border_radius + 20.0) % 32.0;
        self.width = (self.width + 69.0) % 200.0 + 50.0;
        self.height = (self.height + 83.0) % 200.0 + 50.0;
        self.color = Color::from_rgb(
            (self.color.r + 0.3) % 1.0,
            (self.color.g + 0.45) % 1.0,
            (self.color.b + 0.6) % 1.0,
        );
    }
}

#[derive(Debug, Clone)]
enum Message {
    Randomize,
}

struct State {
    rectangle: CustomRect,
}

impl Default for State {
    fn default() -> Self {
        Self {
            rectangle: CustomRect {
                width: 100.0,
                height: 100.0,
                border_radius: 8.0,
                color: Color::from_rgb(1., 0., 0.),
            },
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Randomize => self.rectangle.randomize(),
        }
    }

    fn view(&self) -> impl Widget<Message> + '_ {
        let adjust_button = button(text("Adjust")).on_press(Message::Randomize);

        let animated_box = AnimationBuilder::new(self.rectangle.clone(), |rectangle| {
            container(text(format!(
                "{}x{}",
                rectangle.width as isize, rectangle.height as isize
            )))
            .style(move |_: &iced::Theme| iced::widget::container::Style {
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 1.0,
                    radius: rectangle.border_radius.into(),
                },
                background: Some(rectangle.color.into()),
                ..Default::default()
            })
            .center_x(Length::Fixed(rectangle.width))
            .center_y(Length::Fixed(rectangle.height))
        })
        .animates_layout(true)
        .animation(Easing::EASE);

        let label = text("Animated rectangle");

        column![adjust_button, animated_box, label]
            .spacing(8)
            .padding(8)
            .width(Length::Shrink)
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("Derived Animation")
        .run()
}
