//! An example animating a custom theme palette.
use iced::{
    theme::Base,
    widget::{button, text},
    Border, Color, Widget,
};
use iced_anim::{Animate, Animated, Animation, Event};

/// A custom theme used by your application that supports a `Custom` theme
/// to power animations between different variants.
#[derive(Debug, Clone, Default, PartialEq)]
enum Theme {
    #[default]
    Light,
    Dark,
    /// A custom theme with a custom color palette, useful for animations.
    Custom(Palette),
}

impl Theme {
    /// Gets the relevant color palette for the theme.
    fn palette(&self) -> Palette {
        match self {
            Theme::Light => LIGHT_PALETTE,
            Theme::Dark => DARK_PALETTE,
            Theme::Custom(palette) => *palette,
        }
    }
}

// Implement `Animate` trait using the `Theme::Custom` variant for animated values.
impl Animate for Theme {
    fn components() -> usize {
        Palette::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.palette().distance_to(&end.palette())
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut palette = self.palette();
        palette.update(components);
        *self = Theme::Custom(palette);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        let start = start.palette();
        let end = end.palette();
        let mut palette = start;
        palette.lerp(&start, &end, progress);
        *self = Theme::Custom(palette);
    }
}

// A default daemon style for the custom theme.
impl Base for Theme {
    fn base(&self) -> iced::theme::Style {
        let palette = self.palette();
        iced::theme::Style {
            text_color: palette.text,
            background_color: palette.background,
        }
    }

    fn seed(&self) -> Option<iced::theme::palette::Seed> {
        None
    }

    fn default(preference: iced::theme::Mode) -> Self {
        match preference {
            iced::theme::Mode::Light | iced::theme::Mode::None => Theme::Light,
            iced::theme::Mode::Dark => Theme::Dark,
        }
    }

    fn mode(&self) -> iced::theme::Mode {
        match self {
            Theme::Light => iced::theme::Mode::Light,
            Theme::Dark => iced::theme::Mode::Dark,
            Theme::Custom(_) => iced::theme::Mode::None,
        }
    }

    fn name(&self) -> &str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Custom(_) => "Custom",
        }
    }
}

// Implement custom catalogs for some widgets to use the custom theme.
impl button::Catalog for Theme {
    type Class<'a> = ();
    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, _status: button::Status) -> button::Style {
        let palette = self.palette();
        button::Style {
            text_color: palette.text,
            background: Some(palette.primary.into()),
            border: Border::default().rounded(6),
            ..Default::default()
        }
    }
}

impl text::Catalog for Theme {
    type Class<'a> = ();
    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>) -> text::Style {
        text::Style {
            color: Some(self.palette().text),
            selection: Some(self.palette().background),
        }
    }

    fn selection(&self) -> Color {
        self.palette().background
    }
}

/// A custom color palette used by your application that derives `Animate`.
/// This palette is what you should use in your custom catalog implementations
/// because it gets animated when the theme changes.
#[derive(Debug, Clone, Copy, PartialEq, Animate)]
struct Palette {
    background: Color,
    primary: Color,
    text: Color,
}

/// A light theme for your application.
const LIGHT_PALETTE: Palette = Palette {
    background: Color::from_rgb(0.9, 0.9, 0.9),
    primary: Color::from_rgb(0.0, 0.75, 1.0),
    text: Color::BLACK,
};

/// A dark theme for your application.
const DARK_PALETTE: Palette = Palette {
    background: Color::from_rgb(0.1, 0.1, 0.1),
    primary: Color::from_rgb(1.0, 0.5, 0.0),
    text: Color::WHITE,
};

#[derive(Debug, Clone)]
enum Message {
    /// Indicates that the theme should change or is changing.
    ChangeTheme(Event<Theme>),
}

#[derive(Debug, Default, Clone)]
struct State {
    /// The current app theme, which may be an animated value.
    theme: Animated<Theme>,
}

impl State {
    fn theme(&self) -> Theme {
        self.theme.value().clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTheme(event) => self.theme.update(event),
        }
    }

    fn view(&self) -> impl Widget<Message, Theme> + '_ {
        Animation::new(
            &self.theme,
            button(text("Change theme"))
                .on_press(Message::ChangeTheme(self.next_theme().into()))
                .padding(8),
        )
        .on_update(Message::ChangeTheme)
    }

    fn next_theme(&self) -> Theme {
        match self.theme.target() {
            Theme::Light => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .theme(State::theme)
        .title("Custom theme")
        .run()
}
