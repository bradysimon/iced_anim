//! A widget that helps you animate a value over time from your state.
//!
//! The main difference between this widget and the `AnimationBuilder` is that
//! this widget allows you to directly change the value stored in your state
//! versus passively animating a value. This may also compose better with other
//! animations due to some limitations around widget-driven animations instead
//! of state-driven animations. Refer to the `AnimationBuilder` docs for more
//! details.
//!
//! # Example
//! ```rust
//! use iced_core::time::Duration;
//! use iced_widget::{button, text, Row};
//! use iced_anim::{Animation, Animated, Event, transition::Easing};
//!
//! # type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, iced_widget::Renderer>;
//!
//! struct State {
//!     size: Animated<f32>,
//! }
//!
//! impl Default for State {
//!     fn default() -> Self {
//!        Self {
//!           size: Animated::new(0.0, Easing::LINEAR.with_duration(Duration::from_millis(300)))
//!       }
//!     }
//! }
//!
//! #[derive(Clone)]
//! enum Message {
//!     UpdateSize(Event<f32>),
//! }
//!
//! impl State {
//!     fn update(&mut self, message: Message) {
//!         match message {
//!             Message::UpdateSize(event) => self.size.update(event),
//!         }
//!     }
//!
//!     fn view(&self) -> Element<'_, Message> {
//!         Row::new()
//!             .push(
//!                 button(text("Change target"))
//!                     .on_press(Message::UpdateSize((self.size.target() + 50.0).into()))
//!             )
//!             .push(
//!                 Animation::new(&self.size, text(self.size.value().to_string()))
//!                     .on_update(Message::UpdateSize)
//!             )
//!            .into()
//!     }
//! }
//! ```
use iced_core::{time::Instant, Element, Rectangle, Widget};

use crate::{Animate, Animated, Event};

/// A widget that helps you animate a value over time from your state.
/// This is useful for animating changes to a widget's appearance or layout
/// where you want to directly change the value stored in your state versus
/// passively animating a value like the `AnimationBuilder`.
pub struct Animation<'a, T: Animate, Message, Theme, Renderer> {
    /// The animated value that will be updated over time.
    animated_value: &'a Animated<T>,
    /// The content that will respond to the animation.
    content: Element<'a, Message, Theme, Renderer>,
    /// The function that will be called when the spring needs to be updated.
    on_update: Option<Box<dyn Fn(Event<T>) -> Message>>,
    /// Whether animations are disabled, in which case the value will be updated
    /// immediately without animating. Useful for reduced motion preferences.
    is_disabled: bool,
}

impl<'a, T, Message, Theme, Renderer> Animation<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: 'a,
{
    /// Creates a new `Animation` with the given `animated_value` and `content`.
    pub fn new(
        animated_value: &'a Animated<T>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            animated_value,
            content: content.into(),
            on_update: None,
            is_disabled: false,
        }
    }

    /// Sets the function that will be called when the spring needs to be updated.
    pub fn on_update<F>(mut self, build_message: F) -> Self
    where
        F: Fn(Event<T>) -> Message + 'static,
    {
        self.on_update = Some(Box::new(build_message));
        self
    }

    /// Whether to disable animations and update the value immediately.
    /// Useful for reduced motion preferences.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Animation<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: 'a,
    Renderer: 'a + iced_core::Renderer,
{
    fn size(&self) -> iced_core::Size<iced_core::Length> {
        self.content.as_widget().size()
    }

    fn diff(&mut self, tree: &mut iced_core::widget::Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn mouse_interaction(
        &self,
        tree: &iced_core::widget::Tree,
        layout: iced_core::Layout,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        state: &mut iced_core::widget::Tree,
        layout: iced_core::Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<()>,
    ) {
        self.content.as_widget_mut().operate(
            &mut state.children[0],
            layout,
            viewport,
            renderer,
            operation,
        );
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::None
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced_core::widget::Tree,
        layout: iced_core::Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced_core::Vector,
        window: iced_core::Size,
    ) -> Vec<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }

    fn layout(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);

        tree.size = tree.children[0].size;
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn update(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        event: &iced_core::Event,
        layout: iced_core::Layout,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        if !self.animated_value.is_animating() {
            return;
        }

        if let Some(on_update) = &self.on_update {
            let event: Event<T> = if self.is_disabled {
                Event::Settle
            } else {
                let now = Instant::now();
                Event::Tick(now)
            };
            shell.publish(on_update(event));
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<Animation<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(animation: Animation<'a, T, Message, Theme, Renderer>) -> Self {
        Self::new(animation)
    }
}

/// A helper function to create an [`Animation`] with a given value.
pub fn animation<'a, T, Message, Theme, Renderer>(
    value: &'a Animated<T>,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Animation<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: 'a,
{
    Animation::new(value, content)
}
