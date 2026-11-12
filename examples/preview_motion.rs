use iced::{
    widget::{button, column, container, pick_list, radio, row, space, stack, text, Column, Row},
    Alignment::Center,
    Border, Length, Widget,
};
use iced_anim::{
    animated::Mode, animation_builder::AnimationBuilder, spring::Motion, transition::Easing,
};

const CIRCLE_DIAMETER: f32 = 50.0;

const MAX_OFFSET: f32 = 200.0;

/// A version of [`Motion`] that has an associated `name` for display purposes.
#[derive(Debug, Clone, PartialEq)]
struct PreviewMotion {
    name: &'static str,
    motion: Motion,
}

impl std::fmt::Display for PreviewMotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PreviewEasing {
    name: &'static str,
    easing: Easing,
}

impl std::fmt::Display for PreviewEasing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

const MOTIONS: [PreviewMotion; 3] = [
    PreviewMotion {
        name: "Smooth",
        motion: Motion::SMOOTH,
    },
    PreviewMotion {
        name: "Snappy",
        motion: Motion::SNAPPY,
    },
    PreviewMotion {
        name: "Bouncy",
        motion: Motion::BOUNCY,
    },
];

const EASINGS: [PreviewEasing; 5] = [
    PreviewEasing {
        name: "Linear",
        easing: Easing::LINEAR,
    },
    PreviewEasing {
        name: "Ease",
        easing: Easing::EASE,
    },
    PreviewEasing {
        name: "Ease In",
        easing: Easing::EASE_IN,
    },
    PreviewEasing {
        name: "Ease Out",
        easing: Easing::EASE_OUT,
    },
    PreviewEasing {
        name: "Ease In Out",
        easing: Easing::EASE_IN_OUT,
    },
];

#[derive(Debug, Clone)]
enum Message {
    SelectAnimationType(bool),
    ToggleOffset,
    ChangeMotion(PreviewMotion),
    ChangeCurve(PreviewEasing),
}

struct State {
    is_spring: bool,
    offset: f32,
    preview_motion: PreviewMotion,
    preview_easing: PreviewEasing,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_spring: true,
            offset: -MAX_OFFSET,
            preview_motion: MOTIONS[0].clone(),
            preview_easing: EASINGS[0].clone(),
        }
    }
}

impl State {
    /// The animation mode the preview should use.
    pub fn animation_mode(&self) -> Mode {
        if self.is_spring {
            Mode::Spring(self.preview_motion.motion)
        } else {
            Mode::Transition(self.preview_easing.easing)
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleOffset => {
                self.offset *= -1.0;
            }
            Message::SelectAnimationType(is_spring) => {
                self.is_spring = is_spring;
            }
            Message::ChangeMotion(motion) => {
                self.preview_motion = motion;
            }
            Message::ChangeCurve(curve) => {
                self.preview_easing = curve;
            }
        }
    }

    fn view(&self) -> impl Widget<Message> + '_ {
        let radio_buttons = column![
            radio(
                "Spring",
                true,
                Some(self.is_spring),
                Message::SelectAnimationType,
            ),
            radio(
                "Transition",
                false,
                Some(self.is_spring),
                Message::SelectAnimationType,
            ),
        ];

        let animation_picker = if self.is_spring {
            pick_list(
                Some(self.preview_motion.clone()),
                MOTIONS,
                PreviewMotion::to_string,
            )
            .on_select(Message::ChangeMotion)
            .boxed()
        } else {
            pick_list(
                Some(self.preview_easing.clone()),
                EASINGS,
                PreviewEasing::to_string,
            )
            .on_select(Message::ChangeCurve)
            .boxed()
        };

        let toggle_button = button(text("Toggle")).on_press(Message::ToggleOffset);
        let buttons = row![animation_picker, toggle_button].spacing(8);
        let header = row![radio_buttons, buttons].spacing(16).align_y(Center);

        let animated_circles = container(
            AnimationBuilder::new(self.offset, |offset| {
                container(
                    stack![
                        track_background(false),
                        track_background(true),
                        // Vertical circle
                        circle_track(offset, false),
                        // Horizontal circle
                        circle_track(offset, true),
                    ]
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
                .center(Length::Fill)
            })
            .animation(self.animation_mode())
            .animates_layout(true),
        )
        .center(Length::Fill);

        container(column![header, animated_circles].spacing(8).padding(8))
            .width(Length::Fill)
            .height(Length::Fill)
    }
}

/// The background circles showing the final animation positions.
fn track_background<'a>(is_horizontal: bool) -> impl Widget<Message> + 'a {
    let width: Length = if is_horizontal {
        Length::Fill
    } else {
        Length::Fixed(CIRCLE_DIAMETER)
    };
    let height: Length = if is_horizontal {
        Length::Fixed(CIRCLE_DIAMETER)
    } else {
        Length::Fill
    };
    let spacer = if is_horizontal {
        space().width(MAX_OFFSET - CIRCLE_DIAMETER)
    } else {
        space().height(MAX_OFFSET - CIRCLE_DIAMETER)
    };

    let track_items = vec![
        circle(None, is_horizontal).boxed(),
        spacer.boxed(),
        circle(None, is_horizontal).boxed(),
    ];

    let track_circles = if is_horizontal {
        Row::with_children(track_items).boxed()
    } else {
        Column::with_children(track_items).boxed()
    };

    container(track_circles)
        .width(width)
        .height(height)
        .center(Length::Fill)
}

/// The circle that animates along the track.
fn circle_track<'a>(offset: f32, is_horizontal: bool) -> impl Widget<Message> + 'a {
    if is_horizontal {
        container(row![
            offset.is_sign_positive().then_some(
                space()
                    .width(Length::Fixed(offset))
                    .height(Length::Fixed(CIRCLE_DIAMETER))
            ),
            circle(Some(offset), is_horizontal),
            offset.is_sign_negative().then_some(
                space()
                    .width(Length::Fixed(-offset))
                    .height(Length::Fixed(CIRCLE_DIAMETER))
            ),
        ])
        .center(Length::Fill)
        .boxed()
    } else {
        container(column![
            offset.is_sign_positive().then_some(
                space()
                    .width(Length::Fixed(CIRCLE_DIAMETER))
                    .height(Length::Fixed(offset))
            ),
            circle(Some(offset), is_horizontal),
            offset.is_sign_negative().then_some(
                space()
                    .width(Length::Fixed(CIRCLE_DIAMETER))
                    .height(Length::Fixed(-offset))
            ),
        ])
        .center(Length::Fill)
        .boxed()
    }
}

/// A circle that animates along the track or a background circle.
fn circle<'a>(offset: Option<f32>, is_horizontal: bool) -> impl Widget<Message> + 'a {
    container(space().width(Length::Fill).height(Length::Fill))
        .style(move |theme: &iced::Theme| {
            let color = if let Some(offset) = offset {
                circle_color(offset, is_horizontal)
            } else {
                theme.seed().text
            };
            iced::widget::container::Style {
                border: Border {
                    color,
                    width: 4.0,
                    radius: (CIRCLE_DIAMETER / 2.0).into(),
                },
                background: offset
                    .map(|offset| circle_color(offset, is_horizontal))
                    .map(Into::into),
                ..Default::default()
            }
        })
        .width(CIRCLE_DIAMETER)
        .height(CIRCLE_DIAMETER)
}

/// Gets a unique color based on the current offset.
fn circle_color(offset: f32, is_horizontal: bool) -> iced::Color {
    let ratio = ((offset + MAX_OFFSET) / MAX_OFFSET / 2.0).clamp(0.0, 1.0);

    if is_horizontal {
        iced::Color::from_rgb(1.0 - ratio, 0.75, ratio)
    } else {
        iced::Color::from_rgb(ratio, 1.0 - ratio, 0.75)
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("Preview Motion")
        .run()
}
