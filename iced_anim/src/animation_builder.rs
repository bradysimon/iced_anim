//! Implicitly animate between value changes.
//!
//! # Example
//!
//! Here's an example of animating an `f32` value in your app state:
//!
//! ```rust
//! #[derive(Default)]
//! struct State {
//!     size: f32,
//! }
//! ```
//!
//! Then in your view, you can use an `AnimationBuilder` to animate the size of a container.
//! The element built within the closure will animate to the new size automatically.
//!
//! ```rust
//! # use iced_widget::{container, text};
//! # use iced_anim::{AnimationBuilder, transition::Easing};
//! # use iced_core::time::Duration;
//! # type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, iced_widget::Renderer>;
//! # struct State {
//! #     size: f32,
//! # }
//! # #[derive(Clone)]
//! # enum Message {}
//! # impl State {
//! #     fn view(&self) -> Element<'_, Message> {
//! AnimationBuilder::new(self.size, |size| {
//!     container(text(size as isize))
//!         .center(size)
//!         .into()
//! })
//! .animates_layout(true)
//! .animation(Easing::LINEAR.with_duration(Duration::from_millis(300)))
//! #   .into()
//! #     }
//! # }
//! ```
//!
//! You can also animate multiple values at once by using a tuple up to length of four:
//!
//! ```rust
//! # use iced_core::Color;
//! # use iced_widget::{container, text};
//! # use iced_anim::AnimationBuilder;
//! # type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, iced_widget::Renderer>;
//!
//! # #[derive(Default)]
//! # struct MyType {
//! #   size: f32,
//! #   color: Color,
//! # }
//! # #[derive(Clone)]
//! # enum Message {}
//! # impl MyType {
//! #   fn view(&self) -> Element<'_, Message> {
//! AnimationBuilder::new((self.size, self.color), |(size, color)| {
//!     container(text(size as isize).color(color))
//!         .center(size)
//!         .into()
//! })
//! # .into()
//! #   }
//! # }
//! ```
//!
//! # `AnimationBuilder` Limitations
//!
//! It might not be easy or possible to pass in non-clonable content like custom
//! elements to `AnimationBuilder`'s closure to due the closure being having to be
//! invoked multiple times to animate between values. Making reusable functions
//! that use this widget and also take a generic element might be difficult.
//!
//! Nested animations also don't work if both properties are actively being
//! animated. One animation at a time will function correctly, but trying to adjust
//! both at the same time leads to the inner property skipping to the final value.
//! Use the `Animation` widget if you need any of these properties.
//!
//! If these limitations apply to you, consider using the `Animation` widget instead.
use crate::{animate::Animate, animated::Mode, Animated};
use iced_core::{
    layout,
    widget::{tree, Tree},
    Element, Length, Rectangle, Size, Widget,
};

/// A widget that implicitly animates a value anytime it changes.
///
/// This is useful for animating changes to a widget's appearance or layout
/// based on some state change. Animations happen automatically when the value
/// changes, and the widget will use spring interpolation to animate the value
/// over a specified duration.
pub struct AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
{
    /// The target value to animate to.
    target: T,
    /// The function that builds the element using the animated value.
    builder: Box<dyn Fn(T) -> Element<'a, Message, Theme, Renderer> + 'a>,
    /// The mode defining how to animate the value.
    mode: Mode,
    /// Whether the layout will be affected by the animated value.
    animates_layout: bool,
    /// Whether animations are disabled, in which case the value will be updated
    /// immediately without animating. Useful for reduced motion preferences.
    is_disabled: bool,
    /// The cached element built using the most recent animated value and `builder`.
    cached_element: Element<'a, Message, Theme, Renderer>,
}

impl<'a, T, Message, Theme, Renderer> AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
{
    /// Creates a new `AnimationBuilder` with the given value and builder function.
    pub fn new(
        target: T,
        builder: impl Fn(T) -> Element<'a, Message, Theme, Renderer> + 'a,
    ) -> Self {
        let element = (builder)(target.clone());
        Self {
            target,
            builder: Box::new(builder),
            cached_element: element,
            mode: Mode::default(),
            animates_layout: false,
            is_disabled: false,
        }
    }

    /// Defines the way the value will animate.
    pub fn animation(mut self, mode: impl Into<Mode>) -> Self {
        self.mode = mode.into();
        self
    }

    /// Indicates whether this widget should invalidate the application layout
    /// when animating between changes.
    ///
    /// This is set to `false` by default for performance reasons, but you may
    /// want to set this to `true` if you're animating a value that affects the
    /// layout of the widget (e.g. its size, text, position, etc).
    pub fn animates_layout(mut self, animates_layout: bool) -> Self {
        self.animates_layout = animates_layout;
        self
    }

    /// Whether to disable animations and update the value immediately.
    /// Useful for reduced motion preferences.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

impl<'a, T, Message, Theme, Renderer> From<AnimationBuilder<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(animation: AnimationBuilder<'a, T, Message, Theme, Renderer>) -> Self {
        Self::new(animation)
    }
}

struct State<T> {
    animation: Animated<T>,
    mode: Mode,
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for AnimationBuilder<'_, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.cached_element.as_widget().size()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            animation: Animated::new(self.target.clone(), self.mode),
            mode: self.mode,
        })
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<T>>()
    }

    fn diff(&mut self, tree: &mut Tree) {
        // Update the spring's target if it has changed
        let state = tree.state.downcast_mut::<State<T>>();
        if state.animation.target() != &self.target {
            if self.is_disabled {
                state.animation.settle();
            } else {
                state.animation.set_target(self.target.clone());
            }
        }

        if state.mode != self.mode {
            state.mode = self.mode;
            state.animation.apply(self.mode);
        }

        tree.diff_children(std::slice::from_mut(&mut self.cached_element));
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.cached_element
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        state: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<()>,
    ) {
        self.cached_element.as_widget_mut().operate(
            &mut state.children[0],
            layout,
            renderer,
            operation,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced_core::Vector,
    ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
        self.cached_element.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.cached_element.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        self.cached_element.as_widget().draw(
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
        tree: &mut Tree,
        event: &iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) {
        self.cached_element.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        let iced_core::Event::Window(iced_core::window::Event::RedrawRequested(now)) = event else {
            return;
        };

        let state = tree.state.downcast_mut::<State<T>>();

        // Request a redraw if the spring has remaining energy
        if state.animation.is_animating() {
            shell.request_redraw();
            // Only invalidate the layout if the user indicates to do so
            if self.animates_layout {
                shell.invalidate_layout();
            }

            // Update the animation and request a redraw
            state.animation.tick(*now);
            self.cached_element = (self.builder)(state.animation.value().clone());
        }
    }
}

/// A helper function to create an `AnimationBuilder` with a given value and builder function.
pub fn animation_builder<'a, T, Message, Theme, Renderer>(
    value: T,
    builder: impl Fn(T) -> Element<'a, Message, Theme, Renderer> + 'a,
) -> AnimationBuilder<'a, T, Message, Theme, Renderer>
where
    T: 'static + Animate,
{
    AnimationBuilder::new(value, builder)
}
