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
//! # use iced_widget::{container, text, Widget};
//! # use iced_anim::{AnimationBuilder, transition::Easing};
//! # use iced_core::time::Duration;
//! # struct State {
//! #     size: f32,
//! # }
//! # #[derive(Clone)]
//! # enum Message {}
//! # impl State {
//! #     fn view(&self) -> impl Widget<Message> + '_ {
//! AnimationBuilder::new(self.size, |size| {
//!     container(text(size as isize))
//!         .center(size)
//! })
//! .animates_layout(true)
//! .animation(Easing::LINEAR.with_duration(Duration::from_millis(300)))
//! #     }
//! # }
//! ```
//!
//! You can also animate multiple values at once by using a tuple up to length of four:
//!
//! ```rust
//! # use iced_core::Color;
//! # use iced_widget::{container, text, Widget};
//! # use iced_anim::AnimationBuilder;
//!
//! # #[derive(Default)]
//! # struct MyType {
//! #   size: f32,
//! #   color: Color,
//! # }
//! # #[derive(Clone)]
//! # enum Message {}
//! # impl MyType {
//! #   fn view(&self) -> impl Widget<Message> + '_ {
//! AnimationBuilder::new((self.size, self.color), |(size, color)| {
//!     container(text(size as isize).color(color))
//!         .center(size)
//! })
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
    widget::{tree, Meta, Tree},
    Length, Rectangle, Size, Widget,
};

/// A widget that implicitly animates a value anytime it changes.
///
/// This is useful for animating changes to a widget's appearance or layout
/// based on some state change. Animations happen automatically when the value
/// changes, and the widget will use spring interpolation to animate the value
/// over a specified duration.
pub struct AnimationBuilder<'a, T, W>
where
    T: 'static + Animate,
{
    /// The target value to animate to.
    target: T,
    /// The function that builds the element using the animated value.
    builder: Box<dyn Fn(T) -> W + 'a>,
    /// The mode defining how to animate the value.
    mode: Mode,
    /// Whether the layout will be affected by the animated value.
    animates_layout: bool,
    /// Whether animations are disabled, in which case the value will be updated
    /// immediately without animating. Useful for reduced motion preferences.
    is_disabled: bool,
    /// The cached element built using the most recent animated value and `builder`.
    cached_element: W,
}

impl<'a, T, W> AnimationBuilder<'a, T, W>
where
    T: 'static + Animate,
{
    /// Creates a new `AnimationBuilder` with the given value and builder function.
    pub fn new(target: T, builder: impl Fn(T) -> W + 'a) -> Self {
        let cached_element = (builder)(target.clone());
        Self {
            target,
            builder: Box::new(builder),
            mode: Mode::default(),
            animates_layout: false,
            is_disabled: false,
            cached_element,
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

struct State<T> {
    animation: Animated<T>,
    mode: Mode,
}

impl<T, W> Meta for AnimationBuilder<'_, T, W> where T: 'static + Animate {}

impl<T, Message, W, Theme, Renderer> Widget<Message, Theme, Renderer> for AnimationBuilder<'_, T, W>
where
    T: 'static + Animate,
    W: Widget<Message, Theme, Renderer>,
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.cached_element.size()
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

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.cached_element
            .layout(&mut tree.children[0], renderer, limits);

        tree.size = tree.children[0].size;
    }

    fn operate(
        &mut self,
        state: &mut Tree,
        layout: layout::Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<()>,
    ) {
        self.cached_element.operate(
            &mut state.children[0],
            layout,
            viewport,
            renderer,
            operation,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced_core::Vector,
        window: Size,
    ) -> Vec<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
        self.cached_element.overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.cached_element
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        self.cached_element.draw(
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
        layout: iced_core::Layout,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) {
        self.cached_element.update(
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
            // The rebuilt element may differ structurally, so its tree must be re-diffed before the next layout.
            tree.diff_children(std::slice::from_mut(&mut self.cached_element));
        }
    }
}

/// A helper function to create an `AnimationBuilder` with a given value and builder function.
pub fn animation_builder<'a, T, W>(
    value: T,
    builder: impl Fn(T) -> W + 'a,
) -> AnimationBuilder<'a, T, W>
where
    T: 'static + Animate,
{
    AnimationBuilder::new(value, builder)
}
